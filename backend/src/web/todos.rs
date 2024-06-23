/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

use crate::web::todos;
use crate::Result;
use crate::{
    ctx::Ctx,
    model::{ModelController, Todo, TodoForCreate},
};
use axum::{
    extract::{FromRef, Path, State},
    routing::{delete, post},
    Json, Router,
};

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Router ] ------------------------------------------  */

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/todos", post(create_todo).get(list_todos))
        .route("/todos/:id", delete(delete_todo).get(get_todo))
        .with_state(app_state)
}
/*  #endregion   ------------------------------- [ Router ] ------------------------------------------  */

/*  #region      ------------------------------- [ REST HANDLERS ] ------------------------------------------  */

async fn create_todo(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(todo_fc): Json<TodoForCreate>,
) -> Result<Json<Todo>> {
    println!("->> {:<12} - create_todo", "HANDLER");
    let todo = mc.create_todo(ctx, todo_fc).await?;
    Ok(Json(todo))
}

async fn list_todos(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Todo>>> {
    println!("->> {:<12} - list_todos", "HANDLER");
    let todos = mc.list_todos(ctx).await?;
    Ok(Json(todos))
}

async fn delete_todo(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Todo>> {
    println!("->> {:<12} - delete_todo", "HANDLER");
    let todo = mc.delete_todo(ctx, id).await?;
    Ok(Json(todo))
}

async fn get_todo(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Todo>> {
    println!("->> {:<12} - get_todo", "HANDLER");
    let todo = mc.get_todo(ctx, id).await?;
    Ok(Json(todo))
}

//TODO: update_todo

/*  #endregion   ------------------------------- [ REST HANDLERS ] ------------------------------------------  */
