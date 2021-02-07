use serde::{
    Deserialize,
    Serialize,
};

use crate::domain::api_handler::{
    client::{
        ApiRequest,
        ApiRequestBuilder,
        Response,
    },
    response::aoe2net::last_match::PlayerLastMatch,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchInfo {}

pub async fn get_from_aoe2net(
    root: String,
    endpoint: String,
    query: Vec<(String, String)>,
) -> eyre::Result<Response<PlayerLastMatch>> {
    let request: ApiRequest = ApiRequestBuilder::default()
        .root(root)
        .endpoint(endpoint)
        .query(query)
        .build()
        .unwrap();

    let response = request.execute::<PlayerLastMatch>().await?;

    Ok(response)
}
