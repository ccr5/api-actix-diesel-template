use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{api::dto::cash_in::NewCashInDTO, domain::services::cash_in::CashInService};

#[get("/{id}")]
pub async fn get_cash_in(
    req: HttpRequest,
    cash_in_service: web::Data<dyn CashInService>,
) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    match cash_in_service.read(id).await {
        Some(res) => HttpResponse::Ok().json(res),
        None => HttpResponse::NotFound().body("Not Found"),
    }
}

#[post("/create")]
pub async fn create_cash_in(
    payload: web::Json<NewCashInDTO>,
    cash_in_service: web::Data<dyn CashInService>,
) -> impl Responder {
    let json_data = payload.into_inner();
    match cash_in_service.create(json_data.into()).await {
        Ok(cash_in) => HttpResponse::Ok().json(cash_in),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_cash_in(
    req: HttpRequest,
    cash_in_service: web::Data<dyn CashInService>,
) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    match cash_in_service.delete(id).await {
        Ok(()) => HttpResponse::Ok().body("Deleted"),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
