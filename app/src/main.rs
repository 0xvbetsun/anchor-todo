#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
use anchor_client::solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use once_cell::sync::Lazy;

static PROGRAM_ID: Lazy<Pubkey> = Lazy::new(||Pubkey::from_str("FsgyMvD4vw6xSMNkFD14gbgRK5kadrZYzF1xGAcj2WfR").unwrap());

#[get("/")]
fn hello() -> String {
    format!("Hello, world! {}", *PROGRAM_ID)
}

fn main() {
    rocket::ignite().mount("/api", routes![hello]).launch();
}
