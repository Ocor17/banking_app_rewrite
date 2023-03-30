pub mod account;
mod checking;
mod credit;
pub mod customer;
pub mod person;
mod savings;
//use argon2;
//use csv;
//use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
//use std::io::{self, BufRead};
//use std::panic;

fn main() {
    println!("Hello, world!");

    let mut mary = person::Person::new(
        "Mary".to_string(),
        "Lamb".to_string(),
        "oct. 10".to_string(),
        434,
        "100 way st.".to_string(),
        "9152763286".to_string(),
        "no@no.no".to_string(),
    );
    mary.set_first_name("first_name".to_string());

    let bob_checking = checking::Checking::new(1, Some(20.0), 500.00);
    let bob_saving = savings::Savings::new(1, 20.0, 500.00);
    let bob_credit = credit::Credit::new(1, 2, 20.0, 100.00, 500.00);
    let bob = account::Account::new(100, Some(bob_checking), Some(bob_saving), Some(bob_credit));
    //let test_acct  = account::Account::new(10, None, None, None);

    let mut bob_customer = customer::Customer::new(mary.clone(), bob, 1, "test".to_string());

    //let bob_check_id = panic::catch_unwind(|| bob.checking().unwrap());

    bob_customer.person().set_first_name("No NAme".to_string());

    println!("{}", bob_customer.person().first_name());

    bob_customer.person().set_first_name("test".to_string());

    println!("{}", bob_customer.person().first_name());

    println!("{}", bob_customer.account().account_num());
    bob_customer.account().set_account_num(500);

    println!("{}", bob_customer.account().account_num());

    if let Some(checking) = &mut bob_customer.account().checking() {
        checking.set_balance(1000.0);
    }

    println!(
        "{}",
        bob_customer
            .account()
            .checking()
            .as_mut()
            .unwrap()
            .balance()
    );
    /*match bob_check_id {
        Ok(value) => println!("ID:{}", value.account_num()),
        Err(error) => println!("No Checking Account error:{:?}", error),
    }*/

    println!(
        "{} and {}",
        bob_customer.person().first_name(),
        bob_customer.password()
    );

    //customer::Customer::csv_to_arr();
    bob_customer.print_all_fields();
    bob_customer.print_all_balances();
}
