use std::time::Duration;

use aoe2net::endpoints::last_match::*;
use transparencies_backend_rs::domain::api_handler::client_new::A2NClient;

/// Our app name as USERAGENT for the clients
pub(crate) static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Timeout for http-client requests
pub(crate) static CLIENT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
/// Timeout for http-connections
pub(crate) static CLIENT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() {
    let base_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .timeout(CLIENT_REQUEST_TIMEOUT)
        .connect_timeout(CLIENT_CONNECTION_TIMEOUT)
        .use_rustls_tls()
        .https_only(true)
        .build()
        .unwrap();

    let client = A2NClient::with_client(base_client);

    let req = GetLastMatchRequest::builder()
        .game("aoe2de")
        .profile_id("196240")
        .build();

    let response = client.req_get(req).await;

    println!("{:?}", response);
}
