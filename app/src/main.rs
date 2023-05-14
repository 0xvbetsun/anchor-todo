#![feature(decl_macro)]
mod api;
mod domain;
mod repository;

#[macro_use]
extern crate rocket;

fn main() {
    rocket::ignite()
        .mount("/api", routes![api::health::check, api::list::create])
        .launch();
}
