use axum::{
    Router,
    body::Body,
    http::{Request, Response},
};
use backend::{
    server::{HttpServer, HttpServerConfig},
    service::BlogService,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{fmt::Display, sync::Once};
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

static INIT: Once = Once::new();

pub struct TestApp {
    router: Router,
    _fixture: TestFixture,
}

impl TestApp {
    pub async fn new() -> Self {
        let fixture = TestFixture::new().await;
        let config = HttpServerConfig { port: "0" };
        let server = HttpServer::try_new(fixture.service.clone(), config)
            .await
            .expect("Failed to create server.");

        Self {
            router: server.router,
            _fixture: fixture,
        }
    }

    pub async fn call(&self, uri: &str, method: Method, body: Option<Value>) -> Response<Body> {
        let body = match &body {
            Some(value) => {
                Body::from(serde_json::to_string(value).expect("Failed to stringiy body."))
            }
            None => Body::from(""),
        };

        self.router
            .clone()
            .oneshot(
                Request::builder()
                    .method(method.to_string().as_str())
                    .uri(uri)
                    .header("Content-Type", "application/json")
                    .body(body)
                    .expect("Failed to build request."),
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to call endpoint {uri}"))
    }

    pub async fn parse_response<T>(&self, resp: Response<Body>) -> T
    where
        T: DeserializeOwned,
    {
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("Failed to parse body.");
        let json_resp: Value = serde_json::from_slice(&body).expect("Failed to stringify body.");
        let result: T = serde_json::from_value(json_resp).expect("Failed to deserialize value.");
        result
    }
}

pub struct TestFixture {
    pub pool: PgPool,
    pub schema: String,
    pub service: BlogService<backend::db::postgres::Postgres>,
    _postgres: ContainerAsync<Postgres>,
}

impl TestFixture {
    pub async fn new() -> Self {
        // Initialize logging once
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_env_filter("info")
                .with_test_writer()
                .init();
        });

        // Create Postgres image with configuration
        let postgres = Postgres::default()
            .with_user("postgres")
            .with_password("postgres")
            .with_db_name("blog_test");

        // Start container asynchronously
        let postgres_container = postgres
            .start()
            .await
            .expect("Failed to initialize Postgres container.");

        let port = postgres_container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to acquire port number");

        // Create unique schema
        let schema = format!("test_{}", Uuid::new_v4().to_string().replace("-", ""));

        // Connect to DB
        let database_url = format!("postgres://postgres:postgres@localhost:{}/blog_test", port);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        // Create schema
        sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema))
            .execute(&pool)
            .await
            .expect("Failed to create schema");

        // Set search path
        sqlx::query(&format!("SET search_path TO {}", schema))
            .execute(&pool)
            .await
            .expect("Failed to set search path");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        // Create repository
        let postgres_repo = backend::db::postgres::Postgres::try_new(&database_url)
            .await
            .expect("Failed to initialize repository.");

        // Create service
        let service = BlogService::new(postgres_repo);

        Self {
            pool,
            schema,
            service,
            _postgres: postgres_container,
        }
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        // Schedule cleanup of schema
        let pool = self.pool.clone();
        let schema = self.schema.clone();

        tokio::spawn(async move {
            let _ = sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema))
                .execute(&pool)
                .await;
        });
    }
}

pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get => "GET",
                Self::Post => "POST",
                Self::Patch => "PATCH",
                Self::Delete => "DELETE",
            }
        )
    }
}
