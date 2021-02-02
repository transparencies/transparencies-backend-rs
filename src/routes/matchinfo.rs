use actix_web::HttpResponse;

pub async fn matchinfo() -> HttpResponse {
    HttpResponse::Ok().finish()
}
