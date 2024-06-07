use crate::entities::cash_flow::cash_in::CashIn;
use crate::external::container::AppState;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

#[get("/{id}")]
pub async fn get_cash_in(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let res = data.cash_in_usecases.lock().unwrap().read(id).unwrap();
    HttpResponse::Ok().json(res)
}

#[post("/create")]
pub async fn create_cash_in(
    payload: web::Json<CashIn>,
    data: web::Data<AppState>,
) -> impl Responder {
    let json_data = payload.into_inner();
    data.cash_in_usecases
        .lock()
        .unwrap()
        .create(json_data, data.cash_in_repository.lock().unwrap())
        .unwrap();
    // Access the JSON data and perform operations
    // ...
    HttpResponse::Ok().json(json_data)
}

#[put("/{id}")]
pub async fn update_cash_in(req: HttpRequest, payload: web::Json<CashIn>) -> impl Responder {
    let json_data = payload.into_inner();
    HttpResponse::Ok().body("PUT /cash_in")
}

#[delete("/{id}")]
pub async fn delete_cash_in(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("DELETE /cash_in")
}
