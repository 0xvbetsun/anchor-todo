#![feature(decl_macro)]
mod api;
mod domain;
mod repository;

#[macro_use]
extern crate rocket;
#[macro_use] 
extern crate rocket_contrib;

use std::sync::Mutex;

use repository::todo::{InMemoryRepository, Repository};

fn main() {
    rocket::ignite()
        .mount(
            "/api",
            routes![api::health::check, api::list::create, api::list::all],
        )
        .manage(Mutex::new(
            Box::new(InMemoryRepository::new()) as Box<dyn Repository>
        ))
        .launch();
}
