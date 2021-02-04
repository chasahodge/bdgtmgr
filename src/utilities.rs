use std::io;
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn read_str() -> String {
    let mut input: String = String::from("");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Some('\n')=input.chars().next_back() {
                input.pop();
            }
            return input;
        },
        Err(_e) => return input
    }
}

pub fn read_int() -> u32 {
    let mut input = String::new();
    let mut value: u32 = 0;
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let trimmed = input.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => value = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    return value;
}

pub fn read_float() -> f32 {
    let mut input = String::new();
    let mut value: f32 = 0.0;
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let trimmed = input.trim();
    match trimmed.parse::<f32>() {
        Ok(i) => value = i,
        Err(..) => println!("this was not a floating point number: {}", trimmed),
    };

    return value;
}

pub fn read_small_int() -> u8 {
    let mut input = String::new();
    let mut value: u8 = 0;
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let trimmed = input.trim();
    match trimmed.parse::<u8>() {
        Ok(i) => value = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    return value;
}
