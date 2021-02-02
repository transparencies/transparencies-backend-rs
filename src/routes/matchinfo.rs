use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MatchInfo {}

#[derive(Serialize, Deserialize)]
pub struct MatchInfoRequest {
    id_type: String,
    id_number: String,
}

#[get("/matchinfo/{id_type}/{id_number}")]
pub async fn matchinfo(obj: web::Path<MatchInfoRequest>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MatchInfoRequest {
        id_type: obj.id_type.to_string(),
        id_number: obj.id_number.to_string(),
    }))
}
