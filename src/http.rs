use std::net::SocketAddr;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use log::debug;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("hello world")
}

#[get("/~{user}/{tail:.*}")]
async fn resource(web::Path((user, tail)): web::Path<(String, String)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("user: {} tail: {:?}", user, tail))
}

pub async fn start_http_server() -> Result<(), anyhow::Error> {
    debug!("Starting HTTP server");

    let addr: SocketAddr = "0.0.0.0:34413".parse().unwrap();
    debug!("Binding to http://{}", addr);

    HttpServer::new(|| App::new().service(index).service(resource))
        .bind(addr)?
        .run()
        .await?;

    Ok(())
}
