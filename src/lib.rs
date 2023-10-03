#![allow(dead_code, unused_variables)]
use rand::prelude::*;

mod database; //--> database.rs
mod auth_utils; //--> auth_utils.rs


pub use auth_utils::models::Credentials;//bring credentials and status into the scop, not use the whole name
use database::Status;

pub fn authenticate (creds: Credentials){
    let timeout = thread_rng().gen_range(100..500);
    println!("The timeout is {timeout}");
    if let Status::Connected = database::connect_to_database()
    {
        auth_utils::login(creds);
    }
}