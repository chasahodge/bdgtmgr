extern crate diesel;

use self::models::Account;
use self::diesel::prelude::*;

fn main() {
    use schema::accounts::dsl::*;

    let connection = establish_connection();
    let results = accounts
        .limit(5)
        .load::<Account>(&connection)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{}", account.account_name);
        println!("----------\n");
        println!("{}", account.account_holder);
        println!("${}\n", account.balance);
    }
}
