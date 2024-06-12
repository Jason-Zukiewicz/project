/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ TYPES ] ------------------------------------------  */

#[derive(Clone, Debug, Serialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TodoForCreate {
    pub title: String,
}

/*  #endregion   ------------------------------- [ TYPES ] ------------------------------------------  */

/*  #region      ------------------------------- [ Model Controller ] ------------------------------------------  */

#[derive(Clone)]
pub struct ModelController {
    todos_store: Arc<Mutex<Vec<Option<Todo>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            todos_store: Arc::default(),
        })
    }
}

// CRUD IMPL for todo
impl ModelController {
    pub async fn create_todo(&self, todo_fc: TodoForCreate) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();

        let id = store.len() as u64;
        let todo = Todo {
            id,
            title: todo_fc.title,
        };
        store.push(Some(todo.clone()));

        Ok(todo)

    }

    pub async fn list_todos(&self) -> Result<Vec<Todo>> {
        let store = self.todos_store.lock().unwrap();

        let todos = store.iter().filter_map(|t| t.clone()).collect();

        Ok(todos)
    }

    pub async fn delete_todo(&self, id: u64) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();
        let todo = store.get_mut(id as usize).and_then(|t| t.take());
        todo.ok_or(Error::TodoDeleteFailIdNotFound {id})
    }

    
    pub async fn get_todo(&self, id: u64) -> Result<Todo> {
        let mut store = self.todos_store.lock().unwrap();
        let todo = store.get_mut(id as usize).and_then(|t| t.clone());
        todo.ok_or(Error::TodoGetFailIdNotFound {id})
    }

    //TODO: 
    pub async fn _update_todo(&self, _id: u64) -> Result<Todo> {
        todo!()
    }


}

/*  #endregion   ------------------------------- [ Model Controller ] ------------------------------------------  */