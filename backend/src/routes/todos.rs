use axum::{
    extract::{FromRef, Path, State},
    http::Response,
    routing::{delete, post},
    Json, Router,
};

use crate::error::Result;
use crate::models::{
    controller::ModelController,
    todos::{Todo, TodoForCreate},
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/todos", post(create_todo).get(list_todos))
        .route("/todos/:id", delete(delete_todo).get(get_todo))
        .with_state(mc)
}

//# ------------------------------------------------------------------

async fn list_todos(State(mc): State<ModelController>) -> Result<Json<Vec<Todo>>> {
    println!("->> {:<12} - list_todo", "ROUTE");
    let todos = mc.list_todos().await?;
    Ok(Json(todos))
}

async fn create_todo(
    State(mc): State<ModelController>,
    Json(todo_fc): Json<TodoForCreate>,
) -> Result<Json<Todo>> {
    println!("->> {:<12} - create_todo", "ROUTE");

    let todo = mc.create_todo(todo_fc).await?;
    Ok(Json(todo))
}

async fn delete_todo(State(mc): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Todo>> {
    println!("->> {:<12} - delete_todo", "ROUTE");
    let todo = mc.delete_todo(id).await?;
    Ok(Json(todo))
}

async fn get_todo(State(mc): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Todo>> {
    println!("->> {:<12} - get_todo", "ROUTE");
    let todo = mc.get_todo(id).await?;
    Ok(Json(todo))
}
