use actix_web::{get, web, HttpResponse, Result, HttpRequest, post };
use serde::{Deserialize, Serialize};
use actix_files::NamedFile;

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

#[get("/api/configs/{name}")]
pub async fn conf(req: HttpRequest) -> Result<HttpResponse> {
    let name: String = req.match_info().get("name").unwrap().parse().unwrap();
    let config = service::get_config(&name)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(config))  
}

#[derive(Serialize, Deserialize)]
pub struct MakeItem {
    pub name: String,
    pub sentences: Vec<String>
}

#[post("/api/make")]
pub async fn make(item: web::Json<MakeItem>) -> Result<NamedFile> {
    let git_path = service::make(&item).unwrap();
    Ok(NamedFile::open(git_path)?)
}

// pub async fn make() -> Result<HttpResponse> {
    // let name: String = req.match_info().get("v1").unwrap().parse().unwrap();
    // let config = service::get_config(&mut name).unwrap();
    // HttpResponse::Ok().body(config)
// }