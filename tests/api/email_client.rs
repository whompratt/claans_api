use claans_api::domain::UserEmail;
use claans_api::email_client::EmailClient;
use claims::{assert_err, assert_ok};
use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::{Fake, Faker};
use secrecy::SecretString;
use wiremock::matchers::{any, header, header_exists, method, path};
use wiremock::Request;
use wiremock::{Mock, MockServer, ResponseTemplate};
struct SendEmailBodyMatcher;

fn subject() -> String {
    Sentence(1..2).fake()
}

fn content() -> String {
    Paragraph(1..10).fake()
}

fn email() -> UserEmail {
    UserEmail::parse(SafeEmail().fake()).unwrap()
}

fn email_client(base_url: String) -> EmailClient {
    EmailClient::new(
        base_url,
        email(),
        SecretString::new(Faker.fake::<String>().into()),
        std::time::Duration::from_millis(200),
    )
}

impl wiremock::Match for SendEmailBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
        if let Ok(body) = result {
            dbg!(&body);
            body.get("From").is_some()
                && body.get("To").is_some()
                && body.get("Subject").is_some()
                && body.get("HtmlBody").is_some()
                && body.get("TextBody").is_some()
        } else {
            false
        }
    }
}

#[tokio::test]
async fn send_email_sends_the_expected_request() {
    // Arrange
    let mock_server = MockServer::start().await;
    let email_client = email_client(mock_server.uri());

    Mock::given(header_exists("X-Postmark-Server-Token"))
        .and(header("Content-Type", "application/json"))
        .and(path("/email"))
        .and(method("POST"))
        .and(SendEmailBodyMatcher)
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Act
    let _ = email_client
        .send_email(&email(), &subject(), &content(), &content())
        .await;
}

#[tokio::test]
async fn send_email_succeeds_if_the_server_returns_200() {
    // Arrange
    let mock_server = MockServer::start().await;
    let email_client = email_client(mock_server.uri());

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Act
    let outcome = email_client
        .send_email(&email(), &subject(), &content(), &content())
        .await;

    // Assert
    assert_ok!(outcome);
}

#[tokio::test]
async fn send_email_fails_if_the_server_returns_500() {
    // Arrange
    let mock_server = MockServer::start().await;
    let email_client = email_client(mock_server.uri());

    Mock::given(any())
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Act
    let outcome = email_client
        .send_email(&email(), &subject(), &content(), &content())
        .await;

    // Assert
    assert_err!(outcome);
}

#[tokio::test]
async fn send_email_times_out_if_the_server_takes_too_long() {
    // Arrange
    let mock_server = MockServer::start().await;
    let email_client = email_client(mock_server.uri());

    let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));
    Mock::given(any())
        .respond_with(response)
        .expect(1)
        .mount(&mock_server)
        .await;

    // Act
    let outcome = email_client
        .send_email(&email(), &subject(), &content(), &content())
        .await;

    // Assert
    assert_err!(outcome);
}
