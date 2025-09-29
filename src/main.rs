#[cfg(feature = "server")]
use {
    std::net::SocketAddr,
    axum::Router,
    sevenmark::{
        config::db_config::DbConfig,
        connection::database::establish_connection,
        state::AppState,
        utils::logger::init_tracing,
        api::api_routes
    },
};

#[cfg(feature = "server")]
pub async fn run_server() -> anyhow::Result<()> {
    let conn = establish_connection().await;

    let server_url = format!(
        "{}:{}",
        &DbConfig::get().server_host,
        &DbConfig::get().server_port
    );

    let state = AppState {
        conn,
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

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    init_tracing();

    if let Err(err) = run_server().await {
        eprintln!("Application error: {}", err);
    }
}