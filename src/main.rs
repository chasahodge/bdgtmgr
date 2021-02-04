extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod account;
pub mod transaction;
pub mod utilities;

use utilities::*;
use account::*;
use transaction::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let mut done = false;
    while !done {
        println!("\n1 - Create a new Account");
        println!("2 - Print all Accounts");
        println!("3 - Show an Account's Balance");
        println!("4 - Create a Transaction");
        println!("5 - Print all Transaction for a single Accounts");
        println!("0 - Exit");
    
        let choice = read_small_int();
    
        match choice {
            1 => create_new_account(),
            2 => print_all_accounts(),
            3 => show_account_balance(),
            4 => create_new_transaction(),
            5 => print_transactions_for_account(),
            _ => done = true,
        }
    }
}
