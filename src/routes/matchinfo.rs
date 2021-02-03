// use actix_web::{
//     get,
//     web,
//     HttpRequest,
//     HttpResponse,
//     Responder,
//     Result,
// };
// use serde::{
//     Deserialize,
//     Serialize,
// };

// use crate::domain::api_handler::{
//     client::*,
//     response::aoe2net::last_match::PlayerLastMatch,
// };

// #[derive(Serialize, Deserialize)]
// pub struct MatchInfoRequest {
//     id_type: String,
//     id_number: String,
// }

// #[get("/matchinfo/{id_type}/{id_number}")]
// pub async fn matchinfo(
//     obj: web::Path<MatchInfoRequest>
// ) -> Result<HttpResponse> {
//     // TODO: Call into data_processing with MatchInfoRequest as a Parameter
//     let request: ApiRequest = ApiRequestBuilder::default()
//         .root("https://aoe2.net/api/")
//         .endpoint("player/lastmatch")
//         .query(vec![
//             ("game".to_string(), "aoe2de".to_string()),
//             (obj.id_type.to_string(), obj.id_number.to_string()),
//         ])
//         .build()
//         .unwrap();

//     let response = request.execute::<PlayerLastMatch>().await.unwrap();

//     // Respond with Information from data_processing
//     Ok(HttpResponse::Ok().json(response))

//     // Ok(HttpResponse::Ok().json(MatchInfoRequest {
//     //     id_type: obj.id_type.to_string(),
//     //     id_number: obj.id_number.to_string(),
//     // }))
// }
