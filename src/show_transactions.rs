extern crate diesel;

use self::models::Transaction;
use self::diesel::prelude::*;

fn main() {
    use schema::transactions::dsl::*;

    let connection = establish_connection();
    let results = transactions
        .limit(5)
        .load::<Transaction>(&connection)
        .expect("Error loading transactions");

    println!("Displaying {} transactions", results.len());
    for transaction in results {
        println!("{}", transaction.account_name);
        println!("----------\n");
        println!("{}", transaction.occurrance);
        println!("${}\n", transaction.amount);
    }
}
