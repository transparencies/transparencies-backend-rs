use std::convert::TryInto;

/// Query Parameters for [Get Match History](super::get_match_history)
///
/// [`match_history`](https://aoe2.net/#api)
// TODO
use serde_json::Value as JsonValue;

use serde::{
    Deserialize,
    Serialize,
};

use api_client::{
    error::*,
    request::{
        Request,
        RequestGet,
    },
    response::Response,
};

#[derive(
    PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug,
)]
#[non_exhaustive]
pub struct GetMatchHistoryRequest<'a> {
    /// ID of the channel
    #[builder(default = "aoe2de", setter(into))]
    pub game: &'a str,
    #[builder(setter(into))]
    pub leaderboard_id: usize,
    #[builder(default = 1, setter(into))]
    pub start: usize,
    #[builder(default = 1, setter(into))]
    pub count: usize,
    #[builder(default = None, setter(into))]
    pub steam_id: Option<&'a str>,
    #[builder(default = None, setter(into))]
    pub profile_id: Option<&'a str>,
    #[builder(default = None, setter(into))]
    pub steam_ids: Option<Vec<&'a str>>,
    #[builder(default = None, setter(into))]
    pub profile_ids: Option<Vec<&'a str>>,
}

impl<'a> GetMatchHistoryRequest<'a> {
    pub fn new(
        game: &'a str,
        leaderboard_id: usize,
        start: usize,
        count: usize,
        steam_id: Option<&'a str>,
        profile_id: Option<&'a str>,
        steam_ids: Option<Vec<&'a str>>,
        profile_ids: Option<Vec<&'a str>>,
    ) -> GetMatchHistoryRequest<'a> {
        GetMatchHistoryRequest::builder()
            .game(game)
            .leaderboard_id(leaderboard_id)
            .start(start)
            .count(count)
            .steam_id(steam_id)
            .profile_id(profile_id)
            .steam_ids(steam_ids)
            .profile_ids(profile_ids)
            .build()
    }
}

impl<'a> Request for GetMatchHistoryRequest<'a> {
    type Response = Option<JsonValue>;

    const PATH: &'static str = "player/matches";
    const ROOT: &'static str = crate::AOE2_NET_URL;
}

impl<'a> RequestGet for GetMatchHistoryRequest<'a> {
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, Option<JsonValue>>, ApiRequestGetError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            ApiRequestGetError::Utf8Error(
                response.body().clone(),
                e,
                uri.clone(),
            )
        })?;
        // eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(ApiRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<ApiRequestError>(&text)
        {
            return Err(ApiRequestGetError::Error {
                error,
                status: status
                    .try_into()
                    .unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }
        let response: JsonValue = serde_json::from_str(&text).map_err(|e| {
            ApiRequestGetError::DeserializeError(
                text.to_string(),
                e,
                uri.clone(),
            )
        })?;
        Ok(Response {
            data: response.into(),
            pagination: None,
            // pagination: response.pagination.cursor,
            request,
        })
    }
}
