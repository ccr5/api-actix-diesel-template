mod cash_in_routes;

use actix_web::{web, Scope};
use cash_in_routes::cash_in_routes;

pub fn routes() -> Scope {
    web::scope("/api/v1").service(cash_in_routes())
}
