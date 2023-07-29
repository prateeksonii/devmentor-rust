use actix_web::{body::BoxBody, get, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    ok: bool,
}

impl Responder for Response {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    Response { ok: true }
}
