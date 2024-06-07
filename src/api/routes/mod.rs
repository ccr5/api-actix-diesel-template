mod cash_in_routes;
mod cash_out_routes;

use actix_web::{web, Scope};
use cash_in_routes::cash_in_routes;
use cash_out_routes::cash_out_routes;

pub fn routes() -> Scope {
    web::scope("/api/v1")
        .service(cash_in_routes())
        .service(cash_out_routes())
}
