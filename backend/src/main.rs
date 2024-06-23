/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

#![allow(unused)]
#![allow(non_snake_case)]

mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};

use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, get_service, post, Route},
    Json, Router,
};
use ctx::Ctx;
use dotenv::dotenv;
use log::log_request;
use model::ModelController;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use uuid::Uuid;

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Layers ] ------------------------------------------  */

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get Response Error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!("  ->> client_error_body: {client_error_body}");

            // Build new response from client error
            (*status_code, Json(client_error_body)).into_response()
        });

    //# LOG
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
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
    let BACKEND_ADDRESS = format!(
        "{}{}",
        "0.0.0.0:",
        std::env::var("BACKEND_PORT").expect("BACKEND_PORT must be set.")
    );

    //$ Open Cors To Allow the front and back to talk
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    //$ Initialize ModelController AKA Database
    let mc = ModelController::new().await?;

    //$ Create Routing
    let app = web::all_routes(mc.clone())
        .layer(cors)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    //$ Put it all together on a listener
    let listener = tokio::net::TcpListener::bind(&BACKEND_ADDRESS)
        .await
        .unwrap();
    println!("->> LISTENING on LocalHost:{BACKEND_ADDRESS}\n");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

/*  #endregion   ------------------------------- [ Main ] ------------------------------------------  */
