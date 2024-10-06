use actix_web::{web, HttpResponse};
use chrono::Utc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{NewUser, UserEmail, UserName},
    email_client::EmailClient,
    startup::ApplicationBaseUrl,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryFrom<FormData> for NewUser {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = UserName::parse(value.name)?;
        let email = UserEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new user",
    skip(form, pool, email_client, base_url),
    fields(
        user_email = %form.email,
        user_name = %form.name
    )
)]
pub async fn register(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> HttpResponse {
    let new_user = match form.0.try_into() {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let user_id = match insert_user(&pool, &new_user).await {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let user_token = generate_user_token();
    if store_token(&pool, user_id, &user_token).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    if send_confirmation_email(&email_client, new_user, &base_url.0, &user_token)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(user_token, pool)
)]
pub async fn store_token(
    pool: &PgPool,
    user_id: Uuid,
    user_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO user_tokens (user_token, user_id)
        VALUES ($1, $2)"#,
        user_token,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(name = "Saving new user to database", skip(new_user, pool))]
pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<Uuid, sqlx::Error> {
    let user_id = Uuid::new_v4();
    sqlx::query!(
        r#"
            INSERT INTO users (id, email, name, registered_at, status)
            VALUES($1, $2, $3, $4, 'pending_confirmation')
        "#,
        user_id,
        new_user.email.as_ref(),
        new_user.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(user_id)
}

#[tracing::instrument(
    name = "Send a confirmation email to a new user",
    skip(email_client, new_user, base_url, user_token)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_user: NewUser,
    base_url: &str,
    user_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!("{}/users/confirm?user_token={}", base_url, user_token);
    let plain_body = format!(
        "Welcome to Claans!\nVisit {} to confirm you account.",
        confirmation_link
    );
    let html_body = &format!(
        "Welcome to Claans!<br />
        Click <a href=\"{}\"here</a> to confirm your account.",
        confirmation_link
    );
    email_client
        .send_email(
            new_user.email,
            "Welcome to Claans!",
            &html_body,
            &plain_body,
        )
        .await
}

fn generate_user_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}
