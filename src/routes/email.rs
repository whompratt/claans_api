use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use anyhow::Context;
use sqlx::PgPool;

use crate::domain::UserEmail;
use crate::email_client::EmailClient;

use super::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PublishError {
    fn status_code(&self) -> StatusCode {
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

struct ConfirmedUser {
    email: UserEmail,
}

#[tracing::instrument(name = "Get confirmed user emails", skip(pool))]
async fn get_confirmed_user_emails(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedUser, anyhow::Error>>, anyhow::Error> {
    let confirmed_user_emails = sqlx::query!(
        r#"
        SELECT email
        FROM users
        WHERE status = 'confirmed'
        "#,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| match UserEmail::parse(r.email) {
        Ok(email) => Ok(ConfirmedUser { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();

    Ok(confirmed_user_emails)
}

pub async fn publish_email(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, PublishError> {
    let users = get_confirmed_user_emails(&pool).await?;
    for user in users {
        match user {
            Ok(user) => {
                email_client
                    .send_email(
                        &user.email,
                        &body.title,
                        &body.content.html,
                        &body.content.text,
                    )
                    .await
                    .with_context(|| format!("Failed to send email to {}", user.email))?;
            }
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    "Skipping a confirmed user. \
                    Their stored contact details are invalid."
                );
            }
        }
    }
    Ok(HttpResponse::Ok().finish())
}
