mod common;

use axum::http::StatusCode;
use backend::api::post::{
    BulkPostResponse, CreatePostRequest as CreatePostRequestDTO, PostResponse,
};
use backend::api::responses::ApiErrorData;
use backend::domain::models::post::{CreatePostRequest, PostBody, PostTitle};
use backend::domain::repository::{CreatePostError, RepositoryError};
use backend::domain::service::{Service, ServiceError};
use common::{Method, TestApp, TestFixture};
use serde_json::json;

#[tokio::test]
async fn test_service_create_post_works() {
    // Arrange
    let fixture = TestFixture::new().await;
    let title = PostTitle::new("Test title");
    let body = PostBody::new("Test body");
    let create_req = CreatePostRequest::new(title.clone(), body.clone());

    // Act
    let res = fixture.service.create_post(&create_req).await;

    // Assert
    assert!(res.is_ok());

    let data = res.unwrap();
    assert_eq!(title, data.title());
    assert_eq!(body, data.body());
}

#[tokio::test]
async fn test_service_create_post_with_duplicate_title_triggers_error() {
    // Arrange
    let fixture = TestFixture::new().await;
    let post_title = PostTitle::new("Duplicate Title Test");
    let post_body = PostBody::new("This is a test post body");
    let create_req = CreatePostRequest::new(post_title, post_body);

    // Act
    let first_result = fixture.service.create_post(&create_req).await;

    let duplicate_result = fixture.service.create_post(&create_req).await;

    // Assert
    assert!(first_result.is_ok());
    assert!(duplicate_result.is_err());

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
    let app = TestApp::new().await;
    let body = CreatePostRequestDTO {
        title: "My Test Post".to_string(),
        body: "This is a test post body".to_string(),
    };
    let body_value = json!(body);

    // Act
    let response = app.call("/posts", Method::Post, Some(body_value)).await;

    // Assert - Check status code
    assert_eq!(response.status(), StatusCode::CREATED);

    let data: PostResponse = app.parse_response(response).await;

    assert_eq!(data.title, body.title);
    assert_eq!(data.body, body.body);
}

#[tokio::test]
async fn test_create_post_endpoint_duplicate_title() {
    // Arrange
    let app = TestApp::new().await;

    // Create the request body
    let body = CreatePostRequestDTO {
        title: "Duplicate Title".to_string(),
        body: "This is a test post body".to_string(),
    };

    let body_value = json!(body);

    // First request should succeed
    let first_response = app
        .call("/posts", Method::Post, Some(body_value.clone()))
        .await;

    assert_eq!(first_response.status(), StatusCode::CREATED);

    // Second request with same title should fail
    let duplicate_response = app.call("/posts", Method::Post, Some(body_value)).await;

    // Assert - Check status code for conflict
    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);

    let error_data: ApiErrorData = app.parse_response(duplicate_response).await;

    assert!(error_data.message.contains("Duplicate Title"));
}

#[tokio::test]
async fn test_get_posts_endpoint() {
    // Arrange
    let app = TestApp::new().await;

    let body = CreatePostRequestDTO {
        title: "Title".to_string(),
        body: "Body".to_string(),
    };

    let body_value = json!(body);

    let _ = app.call("/posts", Method::Post, Some(body_value)).await;

    // Act
    let resp = app.call("/posts", Method::Get, None).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);

    let bulk: BulkPostResponse = app.parse_response(resp).await;

    assert_eq!(bulk.data.len(), 1);

    let post = bulk.data.first().unwrap();

    assert_eq!(post.title, body.title);
    assert_eq!(post.body, body.body);
}

#[tokio::test]
async fn test_get_post_by_id_endpoint() {
    // Arrange
    let app = TestApp::new().await;

    let body = CreatePostRequestDTO {
        title: "Title".to_string(),
        body: "Body".to_string(),
    };

    let body_value = json!(body);

    let resp = app.call("/posts", Method::Post, Some(body_value)).await;

    let post: PostResponse = app.parse_response(resp).await;
    let id = post.id;

    // Act
    let resp = app.call(&format!("/posts/{}", id), Method::Get, None).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);

    let post: PostResponse = app.parse_response(resp).await;

    assert_eq!(post.title, body.title);
    assert_eq!(post.body, body.body);
}
