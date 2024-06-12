/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */


#![allow(unused_imports)]
#![allow(non_snake_case)]

mod web;
mod error;
mod model;

pub use self::error::{Error, Result};

use std::net::SocketAddr;
use model::ModelController;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};
use axum::{
    http::Method, middleware, response::Response, routing::{get, get_service, post, Route}, Router
};
use dotenv::dotenv;

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Layers ] ------------------------------------------  */

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

fn routes_static() -> Router {
	Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

/*  #endregion   ------------------------------- [ LAYERS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Main ] ------------------------------------------  */

#[tokio::main]
async fn main() -> Result<()> {
    //$ Get the ENV Vars
    dotenv().ok();
    let BACKEND_ADDRESS = std::env::var("BACKEND_ADDRESS").expect("BACKEND_ADDRESS must be set.");

    //$ Open Cors To Allow the front and back to talk
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    //$ Initialize ModelController AKA Database
    let mc = ModelController::new().await?;


    //$ Create Routing 
    let app = web::all_routes(mc.clone())
        .layer(cors)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    

    //$ Put it all together on a listener 
    let listener = tokio::net::TcpListener::bind(&BACKEND_ADDRESS)
        .await
        .unwrap();
    println!("->> LISTENING on {BACKEND_ADDRESS}\n");
    axum::serve(listener, app.into_make_service()).await.unwrap();

    Ok(())
}
/*  #endregion   ------------------------------- [ Main ] ------------------------------------------  */
