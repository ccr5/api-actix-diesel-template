use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use crate::{api::dto::cash_in::CashInDTO, domain::services::cash_in::CashInService};

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
    payload: web::Json<CashInDTO>,
    cash_in_service: web::Data<dyn CashInService>,
) -> impl Responder {
    let json_data = payload.into_inner();
    match cash_in_service.create(json_data.into()).await {
        Ok(()) => HttpResponse::Ok().body("Created"),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/{id}")]
pub async fn update_cash_in(
    req: HttpRequest,
    payload: web::Json<CashInDTO>,
    cash_in_service: web::Data<dyn CashInService>,
) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let json_data = payload.into_inner();
    match cash_in_service.update(id, json_data.into()).await {
        Ok(()) => HttpResponse::Ok().body("Updated"),
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
