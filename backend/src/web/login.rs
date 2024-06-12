/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

use crate::{web, Error, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use axum::{
    response::{Html, IntoResponse}, routing::{get, post}, Json, Router
};


/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Router ] ------------------------------------------  */

pub fn routes() -> Router {
    Router::new()
    .route("/login", get(get_Login))
    .route("/login", post(post_login))
}

/*  #endregion   ------------------------------- [ Router ] ------------------------------------------  */

/*  #region      ------------------------------- [ Payloads ] ------------------------------------------  */

#[derive(Debug, Deserialize)]
struct LoginPayload  {
    username: String,
    password: String,
}

/*  #endregion    ------------------------------- [ Payloads ] ------------------------------------------  */

/*  #region      ------------------------------- [ Routes ] ------------------------------------------  */

async fn get_Login() -> impl IntoResponse {
    println!("->> {:<12} - get_Login", "HANDLER");
    Html("This is the Login")
}

async fn post_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - post_Login",  "HANDLER");

    //TODO: Implement real db/auth logic
    if payload.username != "user" || payload.password != "pass" {
        return Err(Error::LoginFail);
    }

    // FIXME: Set Cookies - impl real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));


    // Create Success Body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

/* #endregion ------------------------------- [ Routes ] ------------------------------------------  */