use std::fs;
use std::path::Path;
use std::io;
use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
mod controller;
mod service;

// fn test_path() {
//     let path = Path::new("resource");
//     let entires = fs::read_dir(path).unwrap()
//     .map(| res | res.map( | e | e.path())).collect::<Result<Vec<_>, io::Error>>().unwrap();
//     println!("{:?}", entires);
//     println!("{:?}", env::current_exe());
//     println!("Hello, world!");
// }

#[get("/")]
async fn index() -> &'static str {
    "Hello World!\r\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(controller::configs)
    })
    .bind("127.0.0.1:7090")?
    .workers(1)
    .run()
    .await
}
