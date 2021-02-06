use serde::{
    Deserialize,
    Serialize,
};

use crate::domain::api_handler::{
    client::{
        ApiRequest,
        ApiRequestBuilder,
        ApiResponse,
    },
    response::aoe2net::last_match::PlayerLastMatch,
};
// .root("https://aoe2.net/api/")
// .endpoint("player/lastmatch")
// .query(vec![
//     ("game".to_string(), "aoe2de".to_string()),
//     (id_type.to_string(), id_number.to_string()),
// ])

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchInfo {}

pub async fn get_from_aoe2net(
    root: String,
    endpoint: String,
    query: Vec<(String, String)>,
) -> eyre::Result<ApiResponse<PlayerLastMatch>> {
    let request: ApiRequest = ApiRequestBuilder::default()
        .root(root)
        .endpoint(endpoint)
        .query(query)
        .build()
        .unwrap();

    let response = request.execute::<PlayerLastMatch>().await?;

    Ok(response)
}
