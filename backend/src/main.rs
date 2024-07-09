#![allow(unused)]
#![allow(non_snake_case)]

mod error;
mod models;
mod routes;

use axum::{middleware, response::Response, Router};
use error::Result;
use models::controller::ModelController;

const ADDRESS: &str = "0.0.0.0:9999";

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes = routes::routes(mc.clone()).layer(middleware::map_response(main_response_mapper));

    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    println!("->> BACKEND RUNNING -- {ADDRESS}");
    axum::serve(listener, routes).await.unwrap();

    Ok(())
}

// ----------------------------------------------------------------

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}
