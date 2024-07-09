use axum::Router;

use crate::models::controller::ModelController;

mod root;
mod todos;

pub fn routes(mc: ModelController) -> Router {
    let root = root::routes();
    let todos = todos::routes(mc);

    Router::new().merge(root).merge(todos)
}
