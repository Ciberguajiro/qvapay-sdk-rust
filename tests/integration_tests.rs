use qvapay_sdk::models::{AuthLoginRequest, RequestPinRequest};
use qvapay_sdk::{models::CreateInvoiceRequest, Environment, QvaPayClient};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_invoice_success() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "app_id": "test_app",
        "amount": 10.0,
        "description": "Test",
        "remote_id": "123",
        "transaction_uuid": "uuid",
        "expire_at": null,
        "url": "https://qvapay.com/pay/uuid"
    });

    Mock::given(method("POST"))
        .and(path("/v2/create_invoice"))
        .and(header("X-API-Key", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = QvaPayClient::new(Environment::Custom(mock_server.uri()));

    let req = CreateInvoiceRequest {
        amount: 10.0,
        description: "Test".to_string(),
        remote_id: "123".to_string(),
        webhook: None,
        products: None,
        expire_at: None,
    };

    let result = client.create_invoice(&req).await.unwrap();
    assert_eq!(result.url, "https://qvapay.com/pay/uuid");
}

#[tokio::test]
async fn test_api_error_handling() {
    let mock_server = MockServer::start().await;

    let error_body = json!({
        "message": "Invalid API Key"
    });

    Mock::given(method("GET"))
        .and(path("/v2/balance"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_body))
        .mount(&mock_server)
        .await;

    let client = QvaPayClient::new(Environment::Custom(mock_server.uri()));

    let result = client.get_balance().await;

    match result {
        Err(qvapay_sdk::SdkError::Api { status, message }) => {
            assert_eq!(status, 401);
            assert_eq!(message, "Invalid API Key");
        }
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_pin_user() {
    let client = QvaPayClient::new(Environment::Production);

    let result = client
        .request_pin(RequestPinRequest {
            email: "bryanlenier@gmail.com".to_string(),
            password: "Bryanlenier#2003".to_string(),
        })
        .await;

    match result {
        Ok(result) => {
            println!("Respuesta: {}", result.message)
        }
        _ => panic!("Expected Api error"),
    }
}

#[tokio::test]
async fn test_login_user() {
    let client = QvaPayClient::new(Environment::Production);

    let result = client
        .login(&AuthLoginRequest {
            email: "bryanlenier@gmail.com".to_string(),
            password: "Bryanlenier#2003".to_string(),
            remember: Some(false),
            two_factor_code: None,
        })
        .await;

    match result {
        Ok(result) => {
            println!("Respuesta: {}", result.me.username)
        }
        Err(err) => println!("Error: {}", err),
    }
}
