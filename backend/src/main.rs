use backend::{
    config::Config,
    db::postgres::Postgres,
    server::{HttpServer, HttpServerConfig},
    service::BlogService,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::from_env();
    let postgres = Postgres::try_new(&config.database_url).await?;
    let blog_service = BlogService::new(postgres);

    let port_str = config.port.to_string();
    let server_config = HttpServerConfig { port: &port_str };

    let http_server = HttpServer::try_new(blog_service, server_config).await?;

    http_server.run().await
}
