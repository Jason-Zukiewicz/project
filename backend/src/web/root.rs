/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

use crate::{Error, Result};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Router ] ------------------------------------------  */

pub fn routes() -> Router {
    Router::new().route("/", get(get_root))
}

/*  #endregion   ------------------------------- [ Router ] ------------------------------------------  */

/*  #region      ------------------------------- [ Routes ] ------------------------------------------  */

async fn get_root() -> impl IntoResponse {
    println!("->> {:<12} - get_root", "HANDLER");
    Html("This is the root")
}

/*  #endregion   ------------------------------- [ Routes ] ------------------------------------------  */
