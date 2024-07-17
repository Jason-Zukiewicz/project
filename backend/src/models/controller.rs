use crate::error::Result;
use crate::models::todos::{Todo, TodoForCreate};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ModelController {
    pub todo_store: Arc<Mutex<Vec<Option<Todo>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            todo_store: Arc::default(),
        })
    }
}
