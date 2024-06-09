use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{api::dto::cash_out::NewCashOutDTO, domain::services::cash_out::CashOutService};

#[get("/{id}")]
pub async fn get_cash_out(
    req: HttpRequest,
    cash_out_service: web::Data<dyn CashOutService>,
) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    match cash_out_service.read(id).await {
        Some(res) => HttpResponse::Ok().json(res),
        None => HttpResponse::NotFound().body("Not Found"),
    }
}

#[post("/create")]
pub async fn create_cash_out(
    payload: web::Json<NewCashOutDTO>,
    cash_out_service: web::Data<dyn CashOutService>,
) -> impl Responder {
    let json_data = payload.into_inner();
    match cash_out_service.create(json_data.into()).await {
        Ok(()) => HttpResponse::Ok().body("Created"),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_cash_out(
    req: HttpRequest,
    cash_out_service: web::Data<dyn CashOutService>,
) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    match cash_out_service.delete(id).await {
        Ok(()) => HttpResponse::Ok().body("Deleted"),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
