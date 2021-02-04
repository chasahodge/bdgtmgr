use crate::utilities::*;
use diesel::pg::PgConnection;

pub fn post_account<'a>(conn: &PgConnection, account_name: &'a str, account_number: &'a str, account_holder: &'a str, balance: f32) -> Account {
    use schema::accounts;

    let new_account = NewAccount {
        account_name: account_name,
        account_number: account_number,
        account_holder: account_holder,
        balance: balance,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_new_account() {
    println!("Enter the Account Holder's name:");
    let account_holder: String = read_str();
    
    println!("Enter the Name of the Account:");
    let account_name: String = read_str();

    println!("Enter the Account Number:");
    let account_number: String = read_str();

    println!("Enter the Account Balance:");
    let balance: f32 = read_float();

    let connection = establish_connection();

    post_account(&connection, &account_name, &account_number, &account_holder, balance);
    println!("Created account {} {} for {}", account_number, account_number, account_holder);
}

fn show_all_accounts() {
    use schema::accounts::dsl::*;

    let connection = establish_connection();
    let results = accounts
        .load::<Account>(&connection)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{}", account.account_name);
        println!("{}", account.account_number);
        println!("{}", account.account_holder);
        println!("${}\n", account.balance);
        println!("----------\n");
    }
}

pub fn choose_account_menu() -> Account {
    let accounts: Vec<Account> = get_all_accounts();
    let mut account: Account = Account::new();

    if accounts.len() > 0 {
        println!("\nSelect an account:");
        for n in 0..accounts.len() {
            println!("{} - {}", n, accounts[n].name);
        }
        let selection: usize = read_small_int().into();
        account = accounts[selection].clone();
    }

    return account;
}

pub fn get_all_accounts() -> Vec<Account>
{

}

pub fn update_account_balance(acnt_num: u32, amount: f32) {
}

pub fn show_account_balance()
{
    let account: Account = choose_account_menu();
    println!("The balance for {} is ${:.2}", account.name, account.balance);
}

pub fn get_all_transactions_for_account() -> Vec<Transaction>
{
    let account: Account = choose_account_menu();
    let transactions: Vec<Transaction> = get_all_transactions();
    let mut acnt_trans:  Vec<Transaction> = Vec::new();
    if transactions.len() != 0 {
        for transaction in transactions {
            if transaction.account == account.name {
                acnt_trans.push(transaction);
            }
        }
    }

    return acnt_trans;
}

pub fn print_account(account: &Account) {
    println!("\nAccount Holder is {}", account.holder);
    println!("The account is named {}", account.name);
    println!("The account number is: {}", account.account_number);
    println!("The balance of the account is ${:.2}", account.balance);
 }

 pub fn print_all_accounts()
 {
 }
 