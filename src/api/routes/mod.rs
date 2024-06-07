/// This module contains the routes for the cash flow functionality.
mod cash_in_routes;

use actix_web::{web, Scope};
use cash_in_routes::cash_in_routes;

/// Configures the routes for the API.
///
/// This function returns a `Scope` that contains the configured routes for the API.
pub fn routes() -> Scope {
    web::scope("/api/v1").service(cash_in_routes())
}
