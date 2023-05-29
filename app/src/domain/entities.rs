use anchor_lang::prelude::ProgramError;
use arrayref::{array_ref, array_refs};
use serde::Serialize;
use std::str;

#[derive(Clone, Serialize)]
pub enum Status {
    Active,
    Deleted,
}

#[derive(Clone, Serialize)]
pub struct TodoList {
    pub id: u8,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing)]
    pub status: Status,
}

impl TodoList {
    pub fn new(id: u8, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            status: Status::Active,
        }
    }
}

#[derive(Clone)]
pub struct TodoItem {
    pub id: u8,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub status: Status,
}

impl TodoItem {
    pub fn new(id: u8, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            done: false,
            status: Status::Active,
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub username: String,
    pub password: String,
}
