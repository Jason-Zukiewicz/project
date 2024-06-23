/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

pub mod login;
pub mod mw_auth;
pub mod root;
pub mod todos;
use crate::model::ModelController;
use axum::{middleware, Router};

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Router ] ------------------------------------------  */

pub fn all_routes(mc: ModelController) -> Router {
    let base_routes = Router::new().merge(root::routes()).merge(login::routes());

    //$ When we create the router with a middleware, we need to layer it correctly so the middleware is not applied to all routes
    let auth_routes = Router::new()
        .merge(todos::routes(mc))
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    //$ Then we can merge it all together at the end
    base_routes.merge(auth_routes)
}

/*  #endregion   ------------------------------- [ Router ] ------------------------------------------  */

/*  #region      ------------------------------- [ Cookies ] ------------------------------------------  */

pub const AUTH_TOKEN: &str = "auth_token";

/*  #endregion   ------------------------------- [ Cookies ] ------------------------------------------  */
