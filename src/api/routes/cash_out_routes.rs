use crate::api::controllers::cash_out_handler;
use actix_web::{web, Scope};

pub fn cash_out_routes() -> Scope {
    web::scope("/cash_out")
        .service(cash_out_handler::get_cash_out)
        .service(cash_out_handler::create_cash_out)
        .service(cash_out_handler::update_cash_out)
        .service(cash_out_handler::delete_cash_out)
}
