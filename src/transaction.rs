extern crate diesel;

use crate::models::{Account, NewAccount};
use crate::models::{Transaction, NewTransaction};
use crate::utilities::*;

pub fn post_transaction<'a>(conn: &PgConnection, account_name: &'a str, account_number: &'a str, locale: &'a str, occurrance: &'a str, amount: f32) -> Transaction {
    use schema::transactions;

    let new_transaction = NewTransaction {
        account_name: account_name,
        account_number: account_number,
        locale: locale,
        occurrance: occurrance,
        amount: amount,
    };

    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_new_transaction() {
    let account: Account = choose_account_menu();
    let acnt_name = account.name.clone();
    let acnt_num = account.account_number;

    println!("Account for {} has a current balance of {:.2}\n", account.name, account.balance);

    println!("What type of transaction?");
    println!("1 - Debit");
    println!("2 - Credit");
    let choice = read_small_int();

    let mut transaction: Transaction = Transaction::create(acnt_name);
    match choice {
        1 => {
            let neg_amount = transaction.amount * -1.0;
            transaction.amount = neg_amount;
        }
        _ => {}
    }
    
    print_transaction(&transaction);

    update_account_balance(acnt_num, transaction.amount);

    let mut transactions: Vec<Transaction> = get_all_transactions();
    transactions.push(transaction);
    let result = save_transactions_to_file(&transactions);
    result.ok();
}

fn show_all_transactions(acntnum: String) {
    use schema::transactions::dsl::*;

    let connection = establish_connection();
    let results = transactions.filter(account_number.eq(acntnum))
        .load::<Transaction>(&connection)
        .expect("Error loading transactions");

    println!("Displaying {} transactions", results.len());
    for transaction in results {
        println!("{}", transaction.account_name);
        println!("----------\n");
        println!("{}", transaction.locale);
        println!("{}", transaction.occurrance);
        println!("${}\n", transaction.amount);
    }
}

pub fn print_transactions_for_account() {
    let transactions = get_all_transactions_for_account();

    if transactions.len() > 1 {
        
        println!("Transactions for {}", transactions[0].account);

        for transaction in transactions {
            println!("Transaction at {}", transaction.location);
            println!("on {}", transaction.occurrance);
            println!("for {:.2}\n", transaction.amount);
        } 
    } else {
        if transactions.len() == 0 {
            println!("There are no transactions for the selected account");
        } else {
            println!("Transactions for {}", transactions[0].account);
            println!("Transaction at {}", transactions[0].location);
            println!("on {}", transactions[0].occurrance);
            println!("for {:.2}\n", transactions[0].amount);
        }
    }
 }
