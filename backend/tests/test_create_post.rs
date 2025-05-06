mod common;

use backend::domain::models::post::{CreatePostRequest, PostBody, PostTitle};
use backend::domain::repository::{CreatePostError, RepositoryError};
use backend::domain::service::{Service, ServiceError};
use common::TestFixture;

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
