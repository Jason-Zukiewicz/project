#![allow(dead_code)]

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, exp, sign)) => {
            //TODO: TOKEN VALIDATION
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove cookie if something is wrong
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store result into request extension
    req.extensions_mut().insert(result_ctx);
    Ok(next.run(req).await)
}

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

//# Token Format 'user-[user-id].[expiration].[signature]'
//# returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

// #region     ------------------------------- [ Ctx ] ------------------------------------------

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - CTX", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequest)?
            .clone()
    }
}

// #endregion  ------------------------------- [ Ctx ] ------------------------------------------
