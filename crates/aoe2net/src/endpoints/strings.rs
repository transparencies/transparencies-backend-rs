/// Query Parameters for [Get Strings](super::get_channel_information)
///
/// [`strings`](https://aoe2.net/#api)
// TODO
use serde_json::Value as JsonValue;

use serde::{
    Deserialize,
    Serialize,
};

use api_client::request::{
    Request,
    RequestGet,
};

#[derive(
    PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug,
)]
#[non_exhaustive]
pub struct GetApiStringsRequest<'a> {
    /// ID of the channel
    #[builder(setter(into))]
    pub game: &'a str,
    pub language: &'a str,
}

impl<'a> Request for GetApiStringsRequest<'a> {
    type Response = Option<JsonValue>;

    const PATH: &'static str = "strings";
}

impl<'a> RequestGet for GetApiStringsRequest<'a> {
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<
        helix::Response<Self, Option<ChannelInformation>>,
        helix::HelixRequestGetError,
    >
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            helix::HelixRequestGetError::Utf8Error(
                response.body().clone(),
                e,
                uri.clone(),
            )
        })?;
        // eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(helix::HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<helix::HelixRequestError>(&text)
        {
            return Err(helix::HelixRequestGetError::Error {
                error,
                status: status
                    .try_into()
                    .unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }
        let response: helix::InnerResponse<Vec<_>> =
            serde_json::from_str(&text).map_err(|e| {
                helix::HelixRequestGetError::DeserializeError(
                    text.to_string(),
                    e,
                    uri.clone(),
                )
            })?;
        Ok(helix::Response {
            data: response.data.into_iter().next(),
            pagination: response.pagination.cursor,
            request,
        })
    }
}
