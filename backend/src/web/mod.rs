/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

pub mod root;
pub mod todos;
pub mod login;
use axum::Router;
use crate::model::ModelController;

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Router ] ------------------------------------------  */

pub fn all_routes(mc : ModelController) -> Router {
    Router::new()
        .merge(root::routes())
        .merge(todos::routes(mc))
        .merge(login::routes())
        
}

/*  #endregion   ------------------------------- [ Router ] ------------------------------------------  */

/*  #region      ------------------------------- [ Cookies ] ------------------------------------------  */

pub const AUTH_TOKEN: &str = "auth_token";

/*  #endregion   ------------------------------- [ Cookies ] ------------------------------------------  */

