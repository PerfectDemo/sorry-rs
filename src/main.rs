use std::fs;
use std::path::Path;
use std::io;
use std::env;

use actix_web::middleware::Logger;
use env_logger;

use actix_web::{get, web, App, HttpServer, Responder, middleware};
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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(controller::configs)
            .service(controller::conf)
            .service(controller::make)
    })
    .bind("127.0.0.1:7090")?
    .workers(1)
    .run()
    .await
}
