use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use crate::{api::dto::cash_in::CashInDTO, domain::services::cash_in::CashInService};

#[get("/{id}")]
pub async fn get_cash_in(
    req: HttpRequest,
    cash_in_service: web::Data<dyn CashInService>,
) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let res = cash_in_service.read(id).await.unwrap();
    HttpResponse::Ok().json(res)
}

#[post("/create")]
pub async fn create_cash_in(
    payload: web::Json<CashInDTO>,
    cash_in_service: web::Data<dyn CashInService>,
) -> impl Responder {
    let json_data = payload.into_inner();
    cash_in_service.create(json_data.into()).await.unwrap();
    HttpResponse::Ok().body("Ok")
}

#[put("/{id}")]
pub async fn update_cash_in(req: HttpRequest, payload: web::Json<CashInDTO>) -> impl Responder {
    let json_data = payload.into_inner();
    HttpResponse::Ok().body("PUT /cash_in")
}

#[delete("/{id}")]
pub async fn delete_cash_in(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("DELETE /cash_in")
}
