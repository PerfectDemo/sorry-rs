use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

use crate::service;

async fn index(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyObj {
        name: obj.name.to_string(),
    }))
}


#[get("/api/configs")]
pub async fn configs() -> Result<HttpResponse> {
    let configs = service::get_configs();
    Ok(HttpResponse::Ok().json(configs))
}

// pub async fn config() -> Result<HttpResponse> {

// }

// pub async fn make() -> Result<HttpResponse> {

// }