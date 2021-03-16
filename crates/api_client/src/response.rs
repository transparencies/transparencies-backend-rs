use crate::request::Request;
/// Response retrieved from endpoint. Data is the type in [`Request::Response`]
#[derive(PartialEq, Debug)]
pub struct Response<R, D>
where
    R: Request<Response = D>,
    D: serde::de::DeserializeOwned + PartialEq,
{
    /// Twitch's response field for `data`.
    pub data: D,
    // /// A cursor value, to be used in a subsequent request to specify the
    // /// starting point of the next set of results.
    // pub pagination: Option<Cursor>,
    /// The request that was sent, used for [pagination](Paginated).
    pub request: Option<R>,
}

#[derive(PartialEq, serde::Deserialize, Debug)]
struct InnerResponse<D> {
    data: D,
    /* /// A cursor value, to be used in a subsequent request to specify the
     * /// starting point of the next set of results.
     *     #[serde(default)]
     *     pagination: Pagination, */
}

// impl<R, D, T> Response<R, D>
// where
//     R: Request<Response = D>,
//     D: IntoIterator<Item = T> + PartialEq + serde::de::DeserializeOwned,
// {
//     /// Get first result of this response.
//     pub fn first(self) -> Option<T> { self.data.into_iter().next() }
// }

// #[cfg(feature = "client")]
// impl<R, D> Response<R, D>
// where
//     R: Request<Response = D> + Clone + Paginated + RequestGet +
// std::fmt::Debug,     D: serde::de::DeserializeOwned + std::fmt::Debug +
// PartialEq, {
//     /// Get the next page in the responses.
//     pub async fn get_next<'a, C: crate::HttpClient<'a>>(
//         self,
//         client: &'a HelixClient<'a, C>,
//         token: &impl TwitchToken,
//     ) -> Result<Option<Response<R, D>>, ClientRequestError<<C as
// crate::HttpClient<'a>>::Error>>     {
//         if let Some(mut req) = self.request.clone() {
//             if self.pagination.is_some() {
//                 req.set_pagination(self.pagination);
//                 let res = client.req_get(req, token).await.map(Some);
//                 if let Ok(Some(r)) = res {
//                     // FIXME: Workaround for https://github.com/twitchdev/issues/issues/18
//                     if r.data == self.data {
//                         Ok(None)
//                     } else {
//                         Ok(Some(r))
//                     }
//                 } else {
//                     res
//                 }
//             } else {
//                 Ok(None)
//             }
//         } else {
//             // TODO: Make into proper error
//             Err(ClientRequestError::Custom(
//                 "no source request attached".into(),
//             ))
//         }
//     }
// }

// /// Request can be paginated with a cursor
// pub trait Paginated: Request {
//     /// Should returns the current pagination cursor.
//     ///
//     /// # Notes
//     ///
//     /// Pass [`Option::None`] if no cursor is found.
//     fn set_pagination(&mut self, cursor: Option<Cursor>);
// }

// /// A cursor for pagination. This is needed because of how pagination is represented in the [New Twitch API](https://dev.twitch.tv/docs/api)
// #[derive(PartialEq, Deserialize, Debug, Clone, Default)]
// struct Pagination {
//     #[serde(default)]
//     cursor: Option<Cursor>,
// }

// /// A cursor is a pointer to the current "page" in the twitch api pagination
// pub type Cursor = String;
