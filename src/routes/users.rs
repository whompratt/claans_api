use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{NewUser, UserEmail, UserName},
    email_client::EmailClient,
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

#[tracing::instrument(name = "Saving new user to database", skip(new_user, pool))]
pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at, status)
            VALUES($1, $2, $3, $4, 'confirmed')
        "#,
        Uuid::new_v4(),
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
    Ok(())
}

#[tracing::instrument(
    name = "Adding a new user",
    skip(form, pool, email_client),
    fields(
        user_email = %form.email,
        user_name = %form.name
    )
)]
pub async fn register(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> HttpResponse {
    let new_user = match form.0.try_into() {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_user(&pool, &new_user).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    if send_confirmation_email(&email_client, new_user)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Send a confirmation email to a new user",
    skip(email_client, new_user)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_user: NewUser,
) -> Result<(), reqwest::Error> {
    let confirmation_link = "https://claans.com/users/confirm";
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
