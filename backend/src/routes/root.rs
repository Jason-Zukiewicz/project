use axum::{
    extract::{FromRef, Path, State},
    http::Response,
    routing::{delete, get, post},
    Json, Router,
};

use crate::error::Result;
use crate::models::{
    controller::ModelController,
    todos::{Todo, TodoForCreate},
};

pub fn routes() -> Router {
    Router::new().route("/", get(get_root))
}

//# ------------------------------------------------------------------

async fn get_root() -> Result<String> {
    println!("->> {:<12} - get_root", "ROUTE");
    Ok("This is the root".to_string())
}
