use crate::api::routes::routes;
use actix_web::{get, web, App, HttpResponse, Responder};

#[get("/")]
async fn welcome_message() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub fn app() -> App {
    App::new()
        .app_data(web::Data::clone(&data))
        .service(routes())
        .service(welcome_message)
}
