#![feature(decl_macro)]
mod api;
mod domain;
mod repository;

#[macro_use]
extern crate rocket;

use std::sync::Mutex;

use repository::todo::InMemoryRepository;

fn main() {
    rocket::ignite()
        .mount("/api", routes![api::health::check, api::list::create])
        .manage(Mutex::new(Box::new(InMemoryRepository::new())))
        .launch();
}
