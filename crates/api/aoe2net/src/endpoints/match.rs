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
/// Query Parameters for [Get Match](super::get_match)
///
/// [`match`](https://aoe2.net/#api)
// TODO
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(PartialEq,
           typed_builder::TypedBuilder,
           Deserialize,
           Serialize,
           Clone,
           Debug)]
#[non_exhaustive]
pub struct GetMatchRequest<'a> {
    /// ID of the channel
    #[builder(default = "aoe2de", setter(into))]
    pub game: &'a str,
    #[builder(default = None, setter(into))]
    pub uuid: Option<Uuid>,
    #[builder(default = None, setter(into))]
    pub match_id: Option<&'a str>,
}

impl<'a> GetMatchRequest<'a> {
    pub fn new(game: &'a str,
               uuid: Option<Uuid>,
               match_id: Option<&'a str>)
               -> GetMatchRequest<'a> {
        GetMatchRequest::builder().game(game)
                                  .uuid(uuid)
                                  .match_id(match_id)
                                  .build()
    }
}

impl<'a> Request for GetMatchRequest<'a> {
    type Response = Option<JsonValue>;

    const PATH: &'static str = "match";
    const ROOT: &'static str = crate::AOE2_NET_URL;
}

impl<'a> RequestGet for GetMatchRequest<'a> {
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
