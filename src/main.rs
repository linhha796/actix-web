use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct ParamCustom {
    first_number: u32,
}

#[derive(Deserialize)]
struct QueryCustom {
    second_number: u32,
}

#[derive(Deserialize)]
struct BodyCustom {
    third_number: u32,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Con heo OK")
}

#[post("/sum/{first_number}")]
async fn sum(
    param: web::Path<ParamCustom>,
    query: web::Query<QueryCustom>,
    body: web::Json<BodyCustom>,
) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "({} + {}) * {} = {}!",
        param.first_number,
        query.second_number,
        body.third_number,
        (param.first_number + query.second_number) * body.third_number
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(sum))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
