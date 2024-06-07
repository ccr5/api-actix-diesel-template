use crate::api::routes::routes;
use crate::container::Container;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::{get, HttpResponse, Responder};
use actix_web::{App, Error};

#[get("/")]
async fn welcome_message() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub fn app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let container = Container::new();
    let cash_in_service = container.cash_in_service.clone();

    App::new()
        .app_data(Data::new(cash_in_service.clone()))
        .service(routes())
        .service(welcome_message)
}
