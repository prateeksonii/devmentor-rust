use std::{fs::File, io::Write, process::Command};

use actix_web::{body::BoxBody, http::header::ContentType, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    js: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    output: String,
    error: String,
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

#[post("/js")]
pub async fn js_handler(req: web::Json<Request>) -> impl Responder {
    let mut file = File::create("test.js").unwrap();
    file.write_all(req.js.as_bytes()).unwrap();
    let out = Command::new("bun").arg("test.js").output().expect("hmm");
    let out_str = String::from_utf8(out.stdout).unwrap();
    let err = String::from_utf8(out.stderr).unwrap();

    Response {
        output: out_str,
        error: err,
    }
}
