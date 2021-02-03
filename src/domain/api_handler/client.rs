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

use ::serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::HashMap,
    time::Duration,
};

// App-Name as USERAGENT
static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApiResponse<T> {
    response: T,
}

// println!("{:#?}", response);
#[derive(Default, Builder, Debug)]
#[builder(public, setter(into))]
pub struct ApiRequest {
    #[builder(setter(skip))]
    client: reqwest::Client,
    root: String,
    endpoint: String,
    query: Vec<(String, String)>,
}

impl ApiRequest {
    pub fn new() -> Self {
        // Duration for timeouts
        let request_timeout: Duration = Duration::new(5, 0);
        let connection_timeout: Duration = Duration::new(5, 0);

        Self {
            client: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .timeout(request_timeout)
                .connect_timeout(connection_timeout)
                .use_rustls_tls()
                .https_only(true)
                .build()
                .unwrap(),
            root: String::new(),
            endpoint: String::new(),
            query: Vec::new(),
        }
    }

    pub async fn execute<R>(&self) -> eyre::Result<ApiResponse<R>>
    where R: serde::Serialize + for<'de> serde::Deserialize<'de> {
        Ok(ApiResponse {
            response: self
                .client
                .get(&format!("{}{}", &self.root, &self.endpoint))
                .query(&self.query)
                .send()
                .await?
                .json::<R>()
                .await?,
        })
    }
}
