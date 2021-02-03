//! Core client logic of the application

use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use stable_eyre::eyre::{
    eyre,
    Report,
    Result,
    WrapErr,
};

use ::serde::Deserialize;
use std::{
    collections::HashMap,
    time::Duration,
};

use crate::config::cli;

use crate::domain::api_handler::response::aoe2net::last_match::PlayerLastMatch;

// App-Name as USERAGENT
static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

// let response =
//     transparencies_backend_rs::domain::api_handler::client::
// get_from_aoe2net().await?;

// println!("{:#?}", response);
#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct ApiRequest {
    #[builder(pattern = "immutable")]
    client: reqwest::Client,
    #[builder(pattern = "immutable")]
    root: String,
    #[builder(pattern = "immutable")]
    endpoint: String,
    query: Vec<(String, String)>,
}

impl ApiRequest {}

pub async fn get_from_aoe2net() -> eyre::Result<PlayerLastMatch> {
    // Duration for timeouts
    let request_timeout: Duration = Duration::new(5, 0);
    let connection_timeout: Duration = Duration::new(5, 0);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .timeout(request_timeout)
        .connect_timeout(connection_timeout)
        .use_rustls_tls()
        .https_only(true)
        .build()?;

    let request: ApiRequest = ApiRequestBuilder::default()
        .client(client)
        .root("https://aoe2.net/api/")
        .endpoint("player/lastmatch")
        .query(vec![
            ("game".to_string(), "aoe2de".to_string()),
            ("steam_id".to_string(), "76561199003184910".to_string()),
        ])
        .build()
        .unwrap();

    let response = request
        .client
        .get(&format!("{}{}", request.root, request.endpoint))
        .query(&request.query)
        .send()
        .await?
        .json::<PlayerLastMatch>()
        .await?;

    Ok(response)
}
