use reqwest::Client as HttpClient;

#[derive(Clone)]
pub struct AppState {
    pub http_client: HttpClient,
}
