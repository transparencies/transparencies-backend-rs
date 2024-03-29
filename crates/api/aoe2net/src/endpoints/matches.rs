use std::convert::TryInto;

use api_client::{
    error::*,
    request::{
        Request,
        RequestGet,
    },
    response::Response,
};
use serde::{
    Deserialize,
    Serialize,
};
/// Query Parameters for [Get Matches](super::get_matches)
///
/// [`matches`](https://aoe2.net/#api)
// TODO
use serde_json::Value as JsonValue;

#[derive(PartialEq,
           typed_builder::TypedBuilder,
           Deserialize,
           Serialize,
           Clone,
           Debug)]
#[non_exhaustive]
pub struct GetMatchesRequest<'a> {
    /// ID of the channel
    #[builder(default = "aoe2de", setter(into))]
    pub game: &'a str,
    #[builder(default = 1, setter(into))]
    pub count: i32,
    #[builder(default = None, setter(into))]
    pub since: Option<&'a str>,
}

impl<'a> GetMatchesRequest<'a> {
    pub fn new(game: &'a str,
               count: i32,
               since: Option<&'a str>)
               -> GetMatchesRequest<'a> {
        GetMatchesRequest::builder().game(game)
                                    .count(count)
                                    .since(since)
                                    .build()
    }
}

impl<'a> Request for GetMatchesRequest<'a> {
    type Response = Option<JsonValue>;

    const PATH: &'static str = "matches";
    const ROOT: &'static str = crate::AOE2_NET_URL;
}

impl<'a> RequestGet for GetMatchesRequest<'a> {
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>)
        -> Result<Response<Self, Option<JsonValue>>, ApiRequestGetError>
        where Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
                       ApiRequestGetError::Utf8Error(response.body().clone(),
                                                     e,
                                                     uri.clone())
                   })?;

        if let Ok(ApiRequestError { error,
                                    status,
                                    message, }) =
            serde_json::from_str::<ApiRequestError>(&text)
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
        Ok(Response { data: response.into(),
                      pagination: None,
                      // pagination: response.pagination.cursor,
                      request })
    }
}
