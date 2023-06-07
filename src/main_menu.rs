use crate::{
    account, bank_statement,
    checking::{self, Checking},
    credit,
    customer::{self, Customer},
    main_menu, savings,
};
use csv;
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};

#[derive(Debug, Clone, Default)]

//review if MainMenu needs any fields
//Review ALL functions to ensure FULL functionality and error handling
pub struct MainMenu {
    customers: Vec<Customer>,
}

#[derive(PartialEq)]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
    None,
}

pub enum TransactionType {
    InquireBalance,
    PaySomeone,
    Deposit,
    Transfer,
    Withdraw,
    None,
}

impl MainMenu {
    pub fn new(customers: Vec<Customer>) -> Self {
        MainMenu { customers }
    }
    //TODO:
    pub fn bank_manager(mut accounts: Vec<Customer>) {
        //let mut first_name: String;
        //let mut last_name: String;
        let mut payer_id: u32;
        let mut payer_index: u32;
        let mut selection = String::new();

        loop {
            //overwrites current_account to fix borrowing issue from potential previous iterations
            let mut current_account = None;
            selection.clear();
            //add logic to escape when inside  one of the options
            println!("Choose which account to lookup:");
            println!("1. Inquire account by type/number");
            println!("2. First name and Last name");
            println!("3. Inquire all accounts");
            println!("4. Print bank statements");
            println!("5. Print a customer's personal information");
            println!("6. Exit");

            std::io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read input");

            //be consistant on whether match brances are captive with loops
            //especially branch 1
            match selection.trim() {
                "1" => {
                    loop {
                        let mut id_input = String::new();
                        println!("Enter ID number in Full e.g. 00:");
                        std::io::stdout().flush().unwrap();
                        std::io::stdin()
                            .read_line(&mut id_input)
                            .expect("failed to read");

                        match id_input.trim().parse::<i64>() {
                            Ok(parsed_id) => {
                                current_account = {
                                    match accounts
                                        .iter_mut()
                                        .find(|p| p.customer_num() == parsed_id)
                                    {
                                        Some(single_account) => Some(single_account),
                                        None => {
                                            println!("No such account exists");
                                            None
                                        }
                                    }
                                };

                                //payer_id = parsed_id;
                                if current_account.is_some() {
                                    break;
                                }
                                id_input.clear();
                            }
                            Err(_) => {
                                println!("Invalid input, not a integer");
                                std::io::stdout().flush().unwrap();
                                id_input.clear();
                            }
                        }
                    }
                    let current_account = current_account.unwrap();
                    loop {
                        selection.clear();
                        println!("What account type?");
                        println!("1. Checking");
                        println!("2. Savings");
                        println!("3. Credit");
                        println!("4. Exit");

                        std::io::stdin()
                            .read_line(&mut selection)
                            .expect("Failed to read input");

                        match selection.trim() {
                            "1" => main_menu::MainMenu::print_balance(
                                current_account,
                                AccountType::Checking,
                            ),
                            "2" => main_menu::MainMenu::print_balance(
                                current_account,
                                AccountType::Savings,
                            ),
                            "3" => main_menu::MainMenu::print_balance(
                                current_account,
                                AccountType::Credit,
                            ),
                            "4" => break,
                            _ => println!("Invalid input"),
                        }
                    }
                }
                "2" => {
                    let mut first_name = String::new();
                    let mut last_name = String::new();

                    println!("Enter First name");
                    std::io::stdout().flush().unwrap();

                    std::io::stdin()
                        .read_line(&mut first_name)
                        .expect("Error reading input.");

                    println!("Enter Last name");
                    std::io::stdout().flush().unwrap();

                    std::io::stdin()
                        .read_line(&mut last_name)
                        .expect("Error  reading input.");

                    //Using for loop in this section to avoid borrowing issues from presumbly checking two fields of an element
                    current_account = {
                        let mut result: Option<&mut Customer> = None;

                        for accts in accounts.iter_mut() {
                            if accts.person().first_name() == first_name.trim()
                                && accts.person().last_name() == last_name.trim()
                            {
                                result = Some(accts);
                                break;
                            }
                        }
                        result
                    };

                    if current_account.is_some() {
                        let current_account = current_account.unwrap();
                        println!("Account summary");
                        main_menu::MainMenu::print_balance(current_account, AccountType::Checking);
                        main_menu::MainMenu::print_balance(current_account, AccountType::Savings);
                        main_menu::MainMenu::print_balance(current_account, AccountType::Credit);
                    } else {
                        println!("No such person exists");
                    }
                    first_name.clear();
                    last_name.clear();
                }
                "3" => {
                    for i in accounts.iter_mut() {
                        println!("{} {}", i.person().first_name(), i.person().last_name());
                        main_menu::MainMenu::print_balance(i, AccountType::Checking);
                        main_menu::MainMenu::print_balance(i, AccountType::Savings);
                        main_menu::MainMenu::print_balance(i, AccountType::Credit);
                        println!("____________________");
                    }
                }
                "4" => {
                    let mut first_name = String::new();
                    let mut last_name = String::new();

                    println!("Please enter the customer's information");

                    println!("Enter First name");
                    std::io::stdout().flush().unwrap();

                    std::io::stdin()
                        .read_line(&mut first_name)
                        .expect("Error reading input.");

                    println!("Enter Last name");
                    std::io::stdout().flush().unwrap();

                    std::io::stdin()
                        .read_line(&mut last_name)
                        .expect("Error  reading input.");

                    //Using for loop in this section to avoid borrowing issues from presumbly checking two fields of an element
                    current_account = {
                        let mut result: Option<&mut Customer> = None;

                        for accts in accounts.iter_mut() {
                            if accts.person().first_name() == first_name.trim()
                                && accts.person().last_name() == last_name.trim()
                            {
                                result = Some(accts);
                                break;
                            }
                        }
                        result
                    };

                    if current_account.is_some() {
                        bank_statement::BankStatement::create_bank_statement(
                            current_account.unwrap(),
                        );
                    } else {
                        println!("No such person exists");
                    }
                    first_name.clear();
                    last_name.clear();
                }
                "5" => {
                    let mut id_input = String::new();
                    println!("Enter ID number in Full e.g. 00:");
                    std::io::stdout().flush().unwrap();
                    std::io::stdin()
                        .read_line(&mut id_input)
                        .expect("failed to read");

                    match id_input.trim().parse::<i64>() {
                        Ok(parsed_id) => {
                            current_account = {
                                match accounts.iter_mut().find(|p| p.customer_num() == parsed_id) {
                                    Some(single_account) => Some(single_account),
                                    None => {
                                        println!("No such account exists");
                                        None
                                    }
                                }
                            };
                        }
                        Err(_) => {
                            println!("Invalid input, not a integer");
                            std::io::stdout().flush().unwrap();
                            id_input.clear();
                        }
                    };

                    if current_account.is_none() {
                        return;
                    }

                    let current_account = current_account.unwrap();

                    main_menu::MainMenu::print_all_fields(current_account)
                }
                "6" => {
                    println!("Goodbye!");
                    break;
                }
                _ => println!("Invalid input"),
            }
        }
    }

    pub fn main_menu(mut accounts: Vec<Customer>) -> Vec<Customer> {
        let mut selection = String::new();
        let mut first_name = String::new();
        let mut last_name = String::new();
        let mut payer_id_input = String::new();
        //let mut payer_index:i64 = -1;
        //let mut payer_id:i64 = -1;

        //let mut  login_selector = String::new();
        let mut password = String::new();
        let mut current_account = None;

        loop {
            selection.clear();
            //add logic to escape when inside  one of the options
            println!("Choose how to login by entering 1 or 2:");
            println!("1. ID number e.g. 00");
            println!("2. First name and Last name");
            println!("3. Exit");

            std::io::stdout().flush().unwrap();

            std::io::stdin()
                .read_line(&mut selection)
                .expect("Error reading input");

            match selection.trim() {
                "1" => {
                    loop {
                        println!("Enter ID number in Full e.g. 00:");
                        std::io::stdout().flush().unwrap();
                        std::io::stdin()
                            .read_line(&mut payer_id_input)
                            .expect("failed to read");

                        match payer_id_input.trim().parse::<i64>() {
                            Ok(parsed_id) => {
                                current_account = {
                                    match accounts
                                        .iter_mut()
                                        .find(|p| p.customer_num() == parsed_id)
                                    {
                                        Some(single_account) => Some(single_account),
                                        None => {
                                            println!("No such account exists");
                                            None
                                        }
                                    }
                                };

                                //payer_id = parsed_id;
                                if current_account.is_some() {
                                    break;
                                }
                                payer_id_input.clear();
                            }
                            Err(_) => {
                                println!("Invalid input, not a integer");
                                std::io::stdout().flush().unwrap();
                                payer_id_input.clear();
                            }
                        }
                    }
                    break;
                }
                "2" => {
                    loop {
                        println!("Enter First name");
                        std::io::stdout().flush().unwrap();

                        std::io::stdin()
                            .read_line(&mut first_name)
                            .expect("Error reading input.");

                        println!("Enter Last name");
                        std::io::stdout().flush().unwrap();

                        std::io::stdin()
                            .read_line(&mut last_name)
                            .expect("Error  reading input.");

                        //Using for loop in this section to avoid borrowing issues from presumbly checking two fields of an element
                        current_account = {
                            let mut result: Option<&mut Customer> = None;

                            for accts in accounts.iter_mut() {
                                if accts.person().first_name() == first_name.trim()
                                    && accts.person().last_name() == last_name.trim()
                                {
                                    result = Some(accts);
                                    break;
                                }
                            }
                            result
                        };

                        if current_account.is_some() {
                            break;
                        }
                        println!("No such person exists");
                        first_name.clear();
                        last_name.clear();
                    }
                    break;
                }
                "3" => {
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid input!");
                    payer_id_input.clear();
                }
            }
        }
        //println!("ID:{} fn:{} ln:{}", payer_id,first_name,last_name);

        let current_account = current_account.unwrap();

        /*         current_account.set_customer_num(999);
        current_account.set_customer_num(1000);
        current_account.account().set_account_num(20);

        current_account.person().set_first_name("Bobby".to_string());
        current_account.person().set_last_name("Fisher".to_string()); */
        //current_account.unwrap().account().set_account_num(999);
        //let account_mut = current_account.unwrap().account().account_num();

        //println!("new: {} Orig: {}", account_mut, current_account.account().account_num());

        //current_account.person().address();

        println!("Enter  password: ");
        std::io::stdin()
            .read_line(&mut password)
            .expect("Issue reading input");

        //println!("{}",current_account.password());

        if !Customer::password_verify(current_account, password.trim()) {
            println!("Wrong password! Exiting");
            return accounts;
        }
        println!("Correct password... Logging in...");

        loop {
            selection.clear();

            println!("Select what you want to do by typing only the number");
            println!("1. Show balance");
            println!("2. Transfer money");
            println!("3. Deposit money");
            println!("4. Withdraw money");
            println!("5. Pay someone");
            println!("6. Exit");

            std::io::stdin()
                .read_line(&mut selection)
                .expect("error reading input");

            match selection.trim() {
                //Check in original code why these function need parameters other than current_account
                "1" => {
                    Self::balance_sub_menu(current_account, AccountType::None);
                }
                "2" => {
                    Self::transfer_sub_menu(
                        current_account,
                        AccountType::None,
                        AccountType::None,
                        0.00,
                    );
                }
                "3" => {
                    Self::deposit_sub_menu(current_account, 0.0, AccountType::None);
                }
                "4" => {
                    Self::withdraw_sub_menu(current_account, AccountType::None, 0.0);
                }
                "5" => {
                    todo!("Needs to be created :(");
                    Self::pay_someone_sub_menu();
                }
                "6" => {
                    println!("Returning to home page...");
                    let accounts = Self::new_balance_sheet(&mut accounts);
                    return accounts;
                }
                _ => println!("-----------Invalid input!-----------"),
            }
        }
    }

    //TODO: need to modify match statements to handle cases of account types not existing
    pub fn balance_sub_menu(acc: &mut Customer, from_where: AccountType) {
        let mut selection = String::new();

        //from where used for non user inputs
        if from_where != AccountType::None {
            match from_where {
                AccountType::Checking => println!(
                    "Checking balance: ${}",
                    acc.account().checking().unwrap().balance()
                ),
                AccountType::Savings => println!(
                    "Savings balance: ${}",
                    acc.account().savings().unwrap().balance()
                ),
                AccountType::Credit => println!(
                    "Credit balance: ${}",
                    acc.account().credit().unwrap().balance()
                ),
                _ => println!("error reading acount type"),
            }
            return;
        }

        loop {
            //Change to match statement?
            selection.clear();
            if selection.is_empty() {
                println!(
                    "Choose account type:
                1. Checking
                2. Savings
                3. Credit
                4. Exit"
                );

                std::io::stdin()
                    .read_line(&mut selection)
                    .expect("Failed to read input");
            }
            match selection.trim() {
                "1" => println!(
                    "Checking balance: ${}",
                    acc.account().checking().unwrap().balance()
                ),
                "2" => println!(
                    "Savings balance: ${}",
                    acc.account().savings().unwrap().balance()
                ),
                "3" => println!(
                    "Credit balance: ${}",
                    acc.account().credit().unwrap().balance()
                ),
                "4" => break,
                _ => println!("Invalid input"),
            }
        }
    }

    //Fix to have balance actually update and figure out why it isn't updating
    pub fn transfer_sub_menu(
        acc: &mut Customer,
        from_where: AccountType,
        to_where: AccountType,
        amount: f32,
    ) {
        let mut selection = String::new();
        let mut selection_two = String::new();
        let mut transfer_amount = String::new();
        let mut parsed_transfer: f32;
        //let mut account_types = acc.account();

        //let account_balance = account_types.checking().unwrap().balance();

        //account_types.checking().unwrap().set_balance(account_balance);

        match from_where {
            AccountType::Checking => match to_where {
                AccountType::Checking => println!("Cannot transfer to same account"),
                AccountType::Savings => {}
                AccountType::Credit => {}
                AccountType::None => (),
            },
            AccountType::Savings => match to_where {
                AccountType::Checking => {}
                AccountType::Savings => println!("Cannot transfer to same account"),
                AccountType::Credit => {}
                AccountType::None => (),
            },
            AccountType::Credit => match to_where {
                AccountType::Checking => {}
                AccountType::Savings => {}
                AccountType::Credit => println!("Cannot transfer to same account"),
                AccountType::None => (),
            },
            AccountType::None => (),
        }

        loop {
            selection.clear();

            if selection.is_empty() {
                println!("Choose account to transfer FROM:");
                println!("1. checking");
                println!("2. savings");
                println!("3. credit");
                println!("4. exit");

                std::io::stdin()
                    .read_line(&mut selection)
                    .expect("Failed to read input");
            }

            if selection.trim() == "4" {
                break;
            }

            loop {
                println!("Choose account to transfer TO:");
                println!("1. checking");
                println!("2. savings");
                println!("3. credit");
                println!("4. exit");

                selection_two.clear();

                std::io::stdin()
                    .read_line(&mut selection_two)
                    .expect("Failed to read input");

                if selection_two.trim() == "4" {
                    break;
                }

                loop {
                    transfer_amount.clear();
                    if transfer_amount.is_empty() {
                        println!("Enter amount to deposit with decimal");
                        std::io::stdin()
                            .read_line(&mut transfer_amount)
                            .expect("failed to read input");

                        match transfer_amount.trim().parse::<f32>() {
                            Ok(val) if val >= 0.0 => {
                                parsed_transfer = val;
                                break;
                            }
                            Ok(_) => println!("Please give a positive number"),
                            Err(_) => println!("Invalid input. Please give a number"),
                        }
                    }
                }

                //find way to check for accounts existance without cannot borrow error
                /*
                "1" => match acc.account().checking() {
                    Some(checking) => checking.set_balance(checking.balance() + parsed_deposit),
                    None => println!("No Checking account found"),
                }, */

                //learn why using match statements in this way worked when trying to call the methods
                //directly did not.
                match selection.trim() {
                    "1" => {
                        //checking to other
                        match selection_two.trim() {
                            "1" => println!("Cannot transfer to same account"),
                            "2" => {
                                if acc.account().checking().unwrap().balance() > parsed_transfer {
                                    match acc.account().checking() {
                                        Some(checking) => {
                                            checking
                                                .set_balance(checking.balance() - parsed_transfer);
                                            match acc.account().savings() {
                                                Some(savings) => {
                                                    savings.set_balance(
                                                        savings.balance() + parsed_transfer,
                                                    );
                                                }
                                                None => println!("No Savings account found"),
                                            }
                                        }
                                        None => println!("No Checking account found"),
                                    }
                                } else {
                                    println!("Not enough funds");
                                }
                            }
                            "3" => {
                                if acc.account().checking().unwrap().balance() > parsed_transfer {
                                    match acc.account().checking() {
                                        Some(checking) => {
                                            checking
                                                .set_balance(checking.balance() - parsed_transfer);
                                            match acc.account().credit() {
                                                Some(credit) => {
                                                    credit.set_balance(
                                                        credit.balance() + parsed_transfer,
                                                    );
                                                }
                                                None => println!("No Credit account found"),
                                            }
                                        }
                                        None => println!("No Checking account found"),
                                    }
                                } else {
                                    println!("Not enough funds");
                                }
                            }
                            _ => println!("invalid input"),
                        }
                    }
                    "2" => {
                        //savings to other
                        match selection_two.trim() {
                            "1" => {
                                if acc.account().savings().unwrap().balance() > parsed_transfer {
                                    match acc.account().savings() {
                                        Some(savings) => {
                                            savings
                                                .set_balance(savings.balance() - parsed_transfer);
                                            match acc.account().checking() {
                                                Some(checking) => {
                                                    checking.set_balance(
                                                        checking.balance() + parsed_transfer,
                                                    );
                                                }
                                                None => println!("No Checking account found"),
                                            }
                                        }
                                        None => println!("No Savings account found"),
                                    }
                                } else {
                                    println!("Not enough funds");
                                }
                            }
                            "2" => println!("Cannot transfer to same account"),
                            "3" => {
                                if acc.account().savings().unwrap().balance() > parsed_transfer {
                                    match acc.account().savings() {
                                        Some(savings) => {
                                            savings
                                                .set_balance(savings.balance() - parsed_transfer);
                                            match acc.account().credit() {
                                                Some(credit) => {
                                                    credit.set_balance(
                                                        credit.balance() + parsed_transfer,
                                                    );
                                                }
                                                None => println!("No Credit account found"),
                                            }
                                        }
                                        None => println!("No Savings account found"),
                                    }
                                } else {
                                    println!("Not enough funds");
                                }
                            }
                            _ => println!("Invalid input"),
                        }
                    }
                    "3" => {
                        //credit to other
                        match selection_two.trim() {
                            "1" => {
                                if acc.account().credit().unwrap().balance() > parsed_transfer {
                                    match acc.account().credit() {
                                        Some(credit) => {
                                            credit.set_balance(credit.balance() - parsed_transfer);
                                            match acc.account().checking() {
                                                Some(checking) => {
                                                    checking.set_balance(
                                                        checking.balance() + parsed_transfer,
                                                    );
                                                }
                                                None => println!("No Checking account found"),
                                            }
                                        }
                                        None => println!("No Credit account found"),
                                    }
                                } else {
                                    println!("Not enough funds");
                                }
                            }
                            "2" => {
                                if acc.account().credit().unwrap().balance() > parsed_transfer {
                                    match acc.account().credit() {
                                        Some(credit) => {
                                            credit.set_balance(credit.balance() - parsed_transfer);
                                            match acc.account().savings() {
                                                Some(savings) => {
                                                    savings.set_balance(
                                                        savings.balance() + parsed_transfer,
                                                    );
                                                }
                                                None => println!("No Savings account found"),
                                            }
                                        }
                                        None => println!("No Credit account found"),
                                    }
                                } else {
                                    println!("Not enough funds");
                                }
                            }
                            "3" => println!("Cannot transfer to same account"),
                            _ => println!("Invalid input"),
                        }
                    }
                    "4" => break,
                    _ => println!("Invalid input"),
                }
            }
        }
    }

    pub fn deposit_sub_menu(acc: &mut Customer, amount: f32, to_where: AccountType) {
        let mut selection = String::new();
        let mut deposit_amount = String::new();
        let mut parsed_deposit: f32;

        if to_where != AccountType::None {
            match to_where {
                AccountType::Checking => acc
                    .account()
                    .checking()
                    .unwrap()
                    .set_balance(acc.account().checking().unwrap().balance() + amount),
                AccountType::Savings => acc
                    .account()
                    .savings()
                    .unwrap()
                    .set_balance(acc.account().savings().unwrap().balance() + amount),
                AccountType::Credit => acc
                    .account()
                    .credit()
                    .unwrap()
                    .set_balance(acc.account().credit().unwrap().balance() + amount),
                AccountType::None => (),
            }
        }

        loop {
            //Change to match statement?
            selection.clear();
            if selection.is_empty() {
                println!("Choose account type:");
                println!("1. Checking");
                println!("2. Savings");
                println!("3. Credit");
                println!("4. Exit");

                std::io::stdin()
                    .read_line(&mut selection)
                    .expect("Failed to read input");
            }

            if selection.trim() == "4" {
                break;
            }

            loop {
                deposit_amount.clear();
                if deposit_amount.is_empty() {
                    println!("Enter amount to deposit with decimal");
                    std::io::stdin()
                        .read_line(&mut deposit_amount)
                        .expect("failed to read input");

                    match deposit_amount.trim().parse::<f32>() {
                        Ok(val) if val >= 0.0 => {
                            parsed_deposit = val;
                            break;
                        }
                        Ok(_) => println!("Please give a positive number"),
                        Err(_) => println!("Invalid input. Please give a number"),
                    }
                }
            }

            match selection.trim() {
                "1" => match acc.account().checking() {
                    Some(checking) => checking.set_balance(checking.balance() + parsed_deposit),
                    None => println!("No Checking account found"),
                },
                "2" => match acc.account().savings() {
                    Some(savings) => savings.set_balance(savings.balance() + parsed_deposit),
                    None => println!("No Savings account found"),
                },
                "3" => {
                    match acc.account().credit() {
                        Some(credit) => {
                            //multiply by -1 since credit balances are shown as negative
                            if (credit.balance() * -1.0) > parsed_deposit {
                                println!("Cannot deposit more than what is owed");
                            } else {
                                credit.set_balance(credit.balance() + parsed_deposit);
                            }
                        }
                        None => println!("No Credit account found"),
                    }
                }
                //"4"=>break,
                _ => println!("Invalid input"),
            }
        }
    }

    pub fn withdraw_sub_menu(acc: &mut Customer, from_where: AccountType, amount: f32) {
        let mut selection = String::new();
        let mut withdraw_amount = String::new();
        let mut parsed_withdraw: f32;

        if from_where != AccountType::None {}

        loop {
            selection.clear();
            if selection.is_empty() {
                println!(
                    "Choose account type:
                1. Checking
                2. Savings
                3. Exit"
                );

                std::io::stdin()
                    .read_line(&mut selection)
                    .expect("Failed to read input");
            }

            if selection.trim() == "3" {
                break;
            }

            loop {
                withdraw_amount.clear();
                if withdraw_amount.is_empty() {
                    println!("Enter amount to deposit with decimal");
                    std::io::stdin()
                        .read_line(&mut withdraw_amount)
                        .expect("failed to read input");

                    match withdraw_amount.trim().parse::<f32>() {
                        Ok(val) if val >= 0.0 => {
                            parsed_withdraw = val;
                            break;
                        }
                        Ok(_) => println!("Please give a positive number"),
                        Err(_) => println!("Invalid input. Please give a number"),
                    }
                }
            }

            match selection.trim() {
                "1" => match acc.account().checking() {
                    Some(checking) => {
                        if checking.balance() < parsed_withdraw {
                            println!("Not enough funds");
                        } else {
                            checking.set_balance(checking.balance() - parsed_withdraw);
                        }
                    }
                    None => println!("No Checking account found"),
                },
                "2" => match acc.account().savings() {
                    Some(savings) => {
                        if savings.balance() < parsed_withdraw {
                            println!("Not enough funds");
                        } else {
                            savings.set_balance(savings.balance() - parsed_withdraw);
                        }
                    }
                    None => println!("No Savings account found"),
                },
                _ => println!("Invalid input"),
            }
        }
    }

    //TODO!
    pub fn pay_someone_sub_menu() {}

    pub fn transaction_log(
        transaction_type: TransactionType,
        user_name: String,
        other_users_name: String,
        amount: f32,
        from_account: String,
        to_account: String,
    ) {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("transaction_log.txt")
            .unwrap();
        let mut writer = BufWriter::new(file);
        let transaction_message: String;

        match transaction_type {
            TransactionType::InquireBalance => {
                transaction_message =
                    format!("{} inquired their {} balance", user_name, from_account);
                writer.write_all(transaction_message.as_bytes()).unwrap();
            }
            TransactionType::PaySomeone => {
                transaction_message =
                    format!("{} payed ${} to {}.", user_name, amount, other_users_name);
                writer.write_all(transaction_message.as_bytes()).unwrap();
            }
            TransactionType::Deposit => {
                transaction_message = format!(
                    "{} deposited ${} into their {} account.",
                    user_name, amount, from_account
                );
                writer.write_all(transaction_message.as_bytes()).unwrap();
            }
            TransactionType::Transfer => {
                transaction_message = format!(
                    "{} transferred ${} from their {} account to theier {} account.",
                    user_name, amount, from_account, to_account
                );
                writer.write_all(transaction_message.as_bytes()).unwrap();
            }
            TransactionType::Withdraw => {
                transaction_message = format!(
                    "{} withdrew ${} from their {} account",
                    user_name, amount, from_account
                );
                writer.write_all(transaction_message.as_bytes()).unwrap();
            }
            TransactionType::None => println!("No action selected"),
        }
    }

    //Function needs lots of testing from weirdness in for loop
    //unwraps should be modified for error checking
    pub fn new_balance_sheet(accounts: &mut Vec<Customer>) -> Vec<Customer> {
        let mut file = csv::Writer::from_path("new_balance_sheet.csv").unwrap();

        file.write_record([
            "First Name",
            "Last Name",
            "Date of Birth",
            "IdentificationNumber",
            "Address",
            "Phone Number",
            "Checking Account Number",
            "Savings Account Number",
            "Credit Account Number",
            "Checking Starting Balance",
            "Savings Starting Balance",
            "Credit Starting Balance",
            "Password",
            "Email",
            "Credit Max",
        ])
        .unwrap();
        file.flush().unwrap();

        for acct in &mut *accounts {
            //let mut acct = single_account.clone();

            let first_name = acct.person().first_name();
            let last_name = acct.person().last_name();
            let date_of_birth = acct.person().date_of_birth();
            let id_num = acct.person().identification_number();
            let address = acct.person().address();
            let phone_num = acct.person().phone_number();
            let checking_num = acct.account().checking().unwrap().account_num();
            let savings_num = acct.account().savings().unwrap().account_num();
            let credit_num = acct.account().credit().unwrap().account_num();
            let checking_bal = acct.account().checking().unwrap().balance();
            let savings_bal = acct.account().savings().unwrap().balance();
            let credit_bal = acct.account().credit().unwrap().balance();
            let password = acct.password();
            let email = acct.person().email();
            let credit_max = acct.account().credit().unwrap().max_credit();

            file.write_record(&[
                first_name,
                last_name,
                date_of_birth,
                id_num.to_string(),
                address,
                phone_num,
                checking_num.to_string(),
                savings_num.to_string(),
                credit_num.to_string(),
                checking_bal.to_string(),
                savings_bal.to_string(),
                credit_bal.to_string(),
                password,
                email,
                credit_max.to_string(),
            ])
            .unwrap();

            file.flush().unwrap();
        }

        accounts.to_vec()
    }

    pub fn print_all_fields(acc: &mut Customer) {
        println!(
            "Name: {} {}",
            acc.person().first_name(),
            acc.person().last_name()
        );
        println!("DOB: {}", acc.person().date_of_birth());
        println!("Address: {}", acc.person().address());
        println!("ID: {}", acc.person().identification_number());
        println!("Phone Number: {}", acc.person().phone_number());
        println!("Email: {}", acc.person().email());
    }

    //old function parameters (mut accounts: Vec<Customer>, acc_num: usize, account_type: AccountType)
    pub fn print_balance(single_account: &mut Customer, account_type: AccountType) {
        //let single_account = accounts.get_mut(acc_num).unwrap();

        match account_type {
            AccountType::Checking => match single_account.account().checking() {
                Some(checking) => println!("Checking: ${}", checking.balance()),

                None => println!("No Checking Account"),
            },
            AccountType::Savings => match single_account.account().savings() {
                Some(savings) => println!("Savings: ${}", savings.balance()),

                None => println!("No Savings Account"),
            },
            AccountType::Credit => match single_account.account().credit() {
                Some(credit) => println!("Credit: ${}", credit.balance()),

                None => println!("No Credit Account"),
            },
            AccountType::None => (),
        }
    }
}
