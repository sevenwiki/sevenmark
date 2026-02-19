use axum::Router;
use sevenmark_server::database_conn::establish_connection;
use sevenmark_server::logger::init_tracing;
use sevenmark_server::r2_conn::establish_revision_storage_connection;
use sevenmark_server::server_config::ServerConfig;
use sevenmark_server::{AppState, api_routes};
use std::net::SocketAddr;

pub async fn run_server() -> anyhow::Result<()> {
    // Establish database connection
    let conn = establish_connection().await;

    // Establish R2 revision storage connection
    let revision_storage = establish_revision_storage_connection()
        .await
        .expect("Failed to connect to R2 revision storage");

    let server_url = format!(
        "{}:{}",
        &ServerConfig::get().server_host,
        &ServerConfig::get().server_port
    );

    let state = AppState {
        conn,
        revision_storage,
    };

    let app = Router::new()
        .merge(api_routes(state.clone()))
        .with_state(state);

    println!("Starting server at: {}", server_url);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_tracing();

    if let Err(err) = run_server().await {
        eprintln!("Application error: {}", err);
    }
}
