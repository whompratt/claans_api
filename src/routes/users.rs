use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use chrono::Utc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    domain::{NewUser, UserEmail, UserName},
    email_client::EmailClient,
    startup::ApplicationBaseUrl,
};

use super::error_chain_fmt;

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

#[derive(thiserror::Error)]
pub enum RegisterError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for RegisterError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegisterError::ValidationError(_) => StatusCode::BAD_REQUEST,
            RegisterError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
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
) -> Result<HttpResponse, RegisterError> {
    let new_user = form.0.try_into().map_err(RegisterError::ValidationError)?;
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    let user_id = insert_user(&mut transaction, &new_user)
        .await
        .context("Failed to insert a new user in the database")?;
    let user_token = generate_user_token();
    store_token(&mut transaction, user_id, &user_token)
        .await
        .context("Failed to store the confirmation token for a new user")?;
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new user and user_token")?;
    send_confirmation_email(&email_client, new_user, &base_url.0, &user_token)
        .await
        .context("Failed to send a confirmation email to new user")?;
    Ok(HttpResponse::Ok().finish())
}

fn generate_user_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
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
            &new_user.email,
            "Welcome to Claans!",
            html_body,
            &plain_body,
        )
        .await
}

#[tracing::instrument(name = "Saving new user to database", skip(transaction, new_user))]
pub async fn insert_user(
    transaction: &mut Transaction<'_, Postgres>,
    new_user: &NewUser,
) -> Result<Uuid, sqlx::Error> {
    let user_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
            INSERT INTO users (id, email, name, registered_at, status)
            VALUES($1, $2, $3, $4, 'pending_confirmation')
        "#,
        user_id,
        new_user.email.as_ref(),
        new_user.name.as_ref(),
        Utc::now()
    );
    transaction.execute(query).await?;
    Ok(user_id)
}

#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(transaction, user_token)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    user_token: &str,
) -> Result<(), StoreTokenError> {
    let query = sqlx::query!(
        r#"
            INSERT INTO user_tokens (user_token, user_id)
            VALUES ($1, $2)
        "#,
        user_token,
        user_id
    );
    transaction.execute(query).await.map_err(StoreTokenError)?;
    Ok(())
}

pub struct StoreTokenError(sqlx::Error);

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to store a user token"
        )
    }
}
