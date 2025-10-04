#[cfg(feature = "server")]
use {
    axum::Router,
    sevenmark::{
        sevenmark::transform::WikiClient,
        api::api_routes, config::db_config::DbConfig, connection::http::create_http_client,
        state::AppState, utils::logger::init_tracing,
    },
    std::net::SocketAddr,
    tracing::error,
};

#[cfg(feature = "server")]
pub async fn run_server() -> anyhow::Result<()> {
    let http_client = create_http_client().await.map_err(|e| {
        error!("Failed to create HTTP client: {}", e);
        anyhow::anyhow!("HTTP client creation failed: {}", e)
    })?;

    let wiki_base_url = format!(
        "http://{}:{}",
        &DbConfig::get().wiki_server_host,
        &DbConfig::get().wiki_server_port
    );

    let wiki_client = WikiClient::new(http_client, wiki_base_url);

    let server_url = format!(
        "{}:{}",
        &DbConfig::get().server_host,
        &DbConfig::get().server_port
    );

    let state = AppState { wiki_client };

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
