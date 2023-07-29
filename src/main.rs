use actix_web::{App, HttpServer};

mod service;
use service::{js::js_handler, ping::ping};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping).service(js_handler))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
