use axum::Router;
use sevenmark_server::{
    api::api_routes, config::db_config::DbConfig, connection::database::establish_connection,
    state::AppState, utils::logger::init_tracing,
};
use std::net::SocketAddr;

pub async fn run_server() -> anyhow::Result<()> {
    // Establish database connection
    let conn = establish_connection().await;

    let server_url = format!(
        "{}:{}",
        &DbConfig::get().server_host,
        &DbConfig::get().server_port
    );

    let state = AppState { conn };

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
    init_tracing();

    if let Err(err) = run_server().await {
        eprintln!("Application error: {}", err);
    }
}
