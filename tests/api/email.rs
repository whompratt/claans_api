use crate::helpers::{spawn_app, ConfirmationLinks, TestApp};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

async fn create_unconfirmed_user(app: &TestApp) -> ConfirmationLinks {
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed user")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;
    app.post_users(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();
    app.get_confirmation_links(&email_request)
}

async fn create_confirmed_user(app: &TestApp) {
    let confirmation_link = create_unconfirmed_user(app).await;
    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

#[tokio::test]
async fn emails_are_not_sent_to_unconfirmed_users() {
    // Arrange
    let app = spawn_app().await;
    create_unconfirmed_user(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0) // Assertion will occur here
        .mount(&app.email_server)
        .await;

    // Act
    let email_request_body = serde_json::json!({
        "title": "Claans Update",
        "content": {
            "text": "Update body as plain text",
            "html": "<p>Update body as HTML</p>",
        }
    });
    let response = app.post_email(email_request_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn emails_are_delivered_to_confirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_user(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    let email_request_body = serde_json::json!({
        "title": "Claans Update",
        "content": {
            "text": "Update body as plain text",
            "html": "<p>Update body as HTML</p>"
        }
    });
    let response = app.post_email(email_request_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn email_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "content": {
                    "text": "Email body as plain text",
                    "html": "<p>Email body as HTML</p>",
                }
            }),
            "missing title",
        ),
        (
            serde_json::json!({
                "content": {
                    "text": "Email body as plain text",
                },
                "title": "Claans!",
            }),
            "missing html content",
        ),
        (
            serde_json::json!({
                "content": {
                    "html": "<p>Email body as HTML</p>",
                },
                "title": "Claans!",
            }),
            "missing plain content",
        ),
        (
            serde_json::json!({
                "title": "Claans!",
            }),
            "missing content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_email(invalid_body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message,
        );
    }
}

#[tokio::test]
async fn requests_missing_authorization_are_rejected() {
    // Arrange
    let app = spawn_app().await;

    let response = reqwest::Client::new()
        .post(&format!("{}/email", &app.address))
        .json(&serde_json::json!({
            "title": "Email Title",
            "content": {
                "text": "Email body as plain text",
                "html": "<p>Email body as HTML</p>",
            }
        }))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(401, response.status().as_u16());
    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}
