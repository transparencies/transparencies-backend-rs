use crate::domain::api_handler::{
    client::*,
    response::aoe2net::last_match::PlayerLastMatch,
};

pub async fn get_from_aoe2net() -> eyre::Result<ApiResponse<PlayerLastMatch>> {
    let request: ApiRequest = ApiRequestBuilder::default()
        .root("https://aoe2.net/api/")
        .endpoint("player/lastmatch")
        .query(vec![
            ("game".to_string(), "aoe2de".to_string()),
            ("steam_id".to_string(), "76561199003184910".to_string()),
        ])
        .build()
        .unwrap();

    let response = request.execute::<PlayerLastMatch>().await?;

    Ok(response)
}
