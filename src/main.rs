use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use app::app::app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || app())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
