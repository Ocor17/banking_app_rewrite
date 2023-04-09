pub mod account;
mod checking;
mod credit;
pub mod customer;
pub mod person;
mod savings;
pub mod main_menu;
pub mod bank_statement;

//use std::panic;

fn main() {
    println!("Hello, world!");

    let mut test_customers:Vec<customer::Customer> = vec![];

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
    let bob_saving = savings::Savings::new(1, Some(20.0), 500.00);
    let bob_credit = credit::Credit::new( 2, 20.0, Some(100.00), 500.00);
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

    bob_customer.person().set_first_name("Bob".to_string());
    bob_customer.person().set_last_name("Bob".to_string());

    test_customers.push(bob_customer);

    test_customers = main_menu::MainMenu::main_menu(test_customers);

    for mut i in test_customers{
        i.print_all_fields();
    }

    let mut csv_customers = customer::Customer::csv_to_customer_arr().unwrap();

    customer::Customer::create_new_customer(&mut csv_customers);

    let newest_cust = &mut csv_customers.last_mut();

    newest_cust.as_mut().unwrap().print_all_balances();

    for mut customer in csv_customers{
        customer.print_all_fields();
    };




    //bob_customer.print_all_fields();
    //bob_customer.print_all_balances();
}
