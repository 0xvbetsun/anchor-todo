use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct TodoList {
    pub id: u8,
    pub title: String,
    pub description: String,
}

impl TodoList {
    pub fn new(id: u8, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
        }
    }
}

#[derive(Clone)]
pub struct TodoItem {
    pub id: u8,
    pub title: String,
    pub description: String,
    pub done: bool,
}

impl TodoItem {
    pub fn _new(id: u8, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            done: false,
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub id: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}
