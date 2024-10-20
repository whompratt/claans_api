use actix_web::http::header;
use actix_web::http::header::{HeaderMap, HeaderValue};
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use anyhow::Context;
use base64::Engine;
use secrecy::SecretString;
use sqlx::PgPool;

use crate::domain::UserEmail;
use crate::email_client::EmailClient;

use super::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PublishError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PublishError::UnexpectedError(_) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            PublishError::AuthError(_) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
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

// async fn validate_credentials()

pub async fn send_email(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    request: HttpRequest,
) -> Result<HttpResponse, PublishError> {
    let _credentials = basic_authentication(request.headers()).map_err(PublishError::AuthError)?;
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

struct Credentials {
    username: String,
    password: SecretString,
}

fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
    let header_value = headers
        .get("Authorization")
        .context("The 'Authorization' header was missing")?
        .to_str()
        .context("The 'Authorization' header was not a valid UTF8 string")?;
    let base64encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization schema was not 'Basic'")?;
    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64encoded_segment)
        .context("Failed to base64-decode 'Basic' credentails")?;
    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credentials string is not valid UTF8")?;

    let mut credentials = decoded_credentials.splitn(2, ":");
    let username = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth"))?
        .to_string();
    let password = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth"))?
        .to_string();

    Ok(Credentials {
        username,
        password: SecretString::from(password),
    })
}
