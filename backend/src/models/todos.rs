use crate::error::{Error, Result};
use crate::models::controller::ModelController;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoForCreate {
    pub title: String,
}

impl ModelController {
    pub async fn create_todo(&self, todo_fc: TodoForCreate) -> Result<Todo> {
        let mut store = self.todo_store.lock().unwrap();

        let id = store.len() as u64;
        let todo = Todo {
            id,
            title: todo_fc.title,
        };

        store.push(Some(todo.clone()));

        Ok(todo)
    }

    pub async fn list_todos(&self) -> Result<Vec<Todo>> {
        let mut store = self.todo_store.lock().unwrap();

        let todos = store.iter().filter_map(|t| t.clone()).collect();

        Ok(todos)
    }

    pub async fn delete_todo(&self, id: u64) -> Result<Todo> {
        let mut store = self.todo_store.lock().unwrap();
        let todo = store.get_mut(id as usize).and_then(|t| t.take());
        todo.ok_or(Error::TodoDeleteFailIdNotFound { id })
    }

    pub async fn get_todo(&self, id: u64) -> Result<Todo> {
        let mut store = self.todo_store.lock().unwrap();
        let todo = store.get_mut(id as usize).and_then(|t| t.clone());
        todo.ok_or(Error::TodoGetFailIdNotFound { id })
    }
}
