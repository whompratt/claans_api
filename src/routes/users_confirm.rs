use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use super::error_chain_fmt;

#[derive(serde::Deserialize)]
pub struct Parameters {
    user_token: String,
}

#[derive(thiserror::Error)]
pub enum ConfirmationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("There is not user associated with the provided token")]
    UnknownToken,
}

impl std::fmt::Debug for ConfirmationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ConfirmationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnknownToken => StatusCode::UNAUTHORIZED,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(name = "Confirm a pending user", skip(parameters, pool))]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ConfirmationError> {
    let user_id = get_user_id_from_token(&pool, &parameters.user_token)
        .await
        .context("Failed to retrieve user id associated with token provided")?
        .ok_or(ConfirmationError::UnknownToken)?;
    confirm_user(&pool, user_id)
        .await
        .context("Failed to update user status to 'confirmed'")?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Mark user as confirmed", skip(user_id, pool))]
pub async fn confirm_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET status = 'confirmed' WHERE id = $1"#,
        user_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tracing::instrument(name = "Get user_id from token", skip(user_token, pool))]
pub async fn get_user_id_from_token(
    pool: &PgPool,
    user_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT user_id FROM user_tokens \
        WHERE user_token = $1",
        user_token,
    )
    .fetch_optional(pool)
    .await?;
    Ok(result.map(|r| r.user_id))
}
