mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use backend::domain::models::post::{CreatePostRequest, PostBody, PostTitle};
use backend::domain::repository::{CreatePostError, RepositoryError};
use backend::domain::service::{Service, ServiceError};
use backend::server::{HttpServer, HttpServerConfig};
use common::TestFixture;
use serde_json::{Value, json};
use tower::ServiceExt;

#[tokio::test]
async fn test_create_post_works() {
    // Arrange
    let fixture = TestFixture::new().await;
    let title = PostTitle::new("Test title");
    let body = PostBody::new("Test body");

    // Act
    let create_req = CreatePostRequest::new(title.clone(), body.clone());
    let res = fixture.service.create_post(&create_req).await;

    // Assert
    assert!(res.is_ok());

    let data = res.unwrap();
    assert_eq!(title, data.title());
    assert_eq!(body, data.body());
}

#[tokio::test]
async fn test_duplicate_title_triggers_error() {
    // Set up test fixture with a PostgreSQL container
    let fixture = TestFixture::new().await;

    // Create post data
    let title = "Duplicate Title Test";
    let post_title = PostTitle::new(title);
    let post_body = PostBody::new("This is a test post body");
    let create_req = CreatePostRequest::new(post_title, post_body);

    // First creation should succeed
    let first_result = fixture.service.create_post(&create_req).await;
    assert!(first_result.is_ok());

    // Creating a post with the same title should fail with duplicate error
    let duplicate_result = fixture.service.create_post(&create_req).await;
    assert!(duplicate_result.is_err());

    // Check that we got the right error type
    match duplicate_result.unwrap_err() {
        ServiceError::RepositoryError(RepositoryError::CreatePostError(
            CreatePostError::Duplicate { title },
        )) => {
            assert_eq!(title.to_string(), "Duplicate Title Test");
        }
        err => panic!("Expected duplicate error, got: {:?}", err),
    }
}

#[tokio::test]
async fn test_create_post_endpoint() {
    // Arrange
    let fixture = TestFixture::new().await;

    // Create the router with our service
    let config = HttpServerConfig { port: "0" };
    let server = HttpServer::try_new(fixture.service.clone(), config)
        .await
        .expect("Failed to create server");

    let app = server.router.clone();

    // Create the request body
    let request_body = json!({
        "title": "My Test Post",
        "body": "This is a test post body"
    });

    // Act - Send the request to the endpoint
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Check status code
    assert_eq!(response.status(), StatusCode::CREATED);

    // Parse response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();

    // Check response structure
    assert_eq!(json_response["status_code"], 201);
    assert_eq!(json_response["data"]["title"], "My Test Post");
    assert_eq!(json_response["data"]["body"], "This is a test post body");

    // Ensure we have the expected fields
    assert!(json_response["data"]["id"].is_string());
    assert!(json_response["data"]["created_at"].is_string());
}

#[tokio::test]
async fn test_create_post_endpoint_duplicate_title() {
    // Arrange
    let fixture = TestFixture::new().await;

    // Create the router with our service
    let config = HttpServerConfig { port: "0" };
    let server = HttpServer::try_new(fixture.service.clone(), config)
        .await
        .expect("Failed to create server");

    let app = server.router.clone();

    // Create the request body
    let request_body = json!({
        "title": "Duplicate Title",
        "body": "This is a test post body"
    });

    // First request should succeed
    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(first_response.status(), StatusCode::CREATED);

    // Second request with same title should fail
    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Check status code for conflict
    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);

    // Parse response body
    let body = axum::body::to_bytes(duplicate_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json_response: Value = serde_json::from_slice(&body).unwrap();

    // Validate error response
    assert_eq!(json_response["status_code"], 409);
    assert!(
        json_response["data"]["message"]
            .as_str()
            .unwrap()
            .contains("Duplicate Title")
    );
}
