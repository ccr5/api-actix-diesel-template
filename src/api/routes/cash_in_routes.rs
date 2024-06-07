use crate::api::controllers::cash_in_handler;
use actix_web::{web, Scope};

pub fn cash_in_routes() -> Scope {
    web::scope("/cash_in")
        .service(cash_in_handler::get_cash_in)
        .service(cash_in_handler::create_cash_in)
}
