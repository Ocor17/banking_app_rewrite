use crate::account::{Account, self};
use crate::{checking, savings, credit, customer};
use crate::person::{Person, self};
use argon2::{self, Argon2};
use csv;
use csv::ReaderBuilder;
use password_hash::{PasswordHasher, SaltString};
use rand::thread_rng;
//use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
//use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone, Default)]
//Move account num here and implement password
pub struct Customer {
    person: Person,
    account: Account,
    customer_num: i64,
    password: String,
}

impl Customer {
    pub fn new(person: Person, account: Account, customer_num: i64, password: String) -> Self {
        let hashed_password = Self::password_hashing(password);
        Customer {
            person: person,
            account: account,
            customer_num: customer_num,
            password: hashed_password,
        }
    }

    pub fn person(&mut self) -> &mut Person {
        &mut self.person
    }
    pub fn set_person(&mut self, person: Person) {
        self.person = person
    }
    pub fn account(&mut self) -> &mut Account {
        &mut self.account
    }
    pub fn set_account(&mut self, account: Account) {
        self.account = account
    }
    pub fn password(&self) -> String {
        self.password.clone()
    }
    pub fn set_password(&mut self, password: String) {
        let hashed_password = Self::password_hashing(password);
        self.password = hashed_password
    }
    pub fn customer_num(&self) -> i64 {
        self.customer_num
    }
    pub fn set_customer_num(&mut self, customer_num: i64) {
        self.customer_num = customer_num
    }

    fn password_hashing(password: String) -> String {
        //use of thread_rng should be checked if this is the best way to get a rand for this implementation.
        let salt = SaltString::generate(&mut thread_rng());
        let argon2 = Argon2::default();
        let new_password = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        new_password
    }

    //currently takes some time to run this function, most likely due to how argon2 works
    pub fn csv_to_customer_arr() -> Result<Vec<Customer>, Box<dyn Error>> {
        let mut input = String::new();
        println!("Input csv filename:");
        io::stdin().read_line(&mut input)?;

        let path = Path::new(input.trim());

        let file = File::open(path)?;

        //expected headers:
        //Savings Account Number[0],Last Name[1],Identification Number[2],
        //Date of Birth[3],Checking Account Number[4],Credit Account Number[5],
        //Phone Number[6],Checking Starting Balance[7],Savings Starting Balance[8],
        //Password[9],Credit Starting Balance[10],Address[11],First Name[12],Email[13],Credit Max[14]

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(file);

        let mut customer_arr = Vec::new();

        //section must be double checked with the original Java code.
        //issue with account num for account and Customer since it exists as id num in person
        //starting balance is passed as none since balance checks for a starting balance and creates one if one does not exis

        for result in reader.records() {
            let record = result?;

            let person = person::Person::new(record[12].to_string(), record[1].to_string(), record[3].to_string(),
             record[2].parse().unwrap(), record[11].to_string(), record[6].to_string(), record[13].to_string());
            let checking = checking::Checking::new(record[4].parse().unwrap(),None , record[7].parse().unwrap());
            let savings = savings::Savings::new(record[0].parse().unwrap(), None, record[8].parse().unwrap());
            let credit = credit::Credit::new(record[5].parse().unwrap(), record[14].parse().unwrap(), None, record[10].parse().unwrap());
            let account = account::Account::new(record[2].parse().unwrap(), Some(checking), Some(savings), Some(credit));
            let customer = Customer::new(person, account, record[2].parse().unwrap(), record[9].to_string());
            
            println!("Creating customer: {} {}...",&record[12],&record[1]);

            customer_arr.push(customer);
            //let record = result?;
            //println!("{:?}", record);
        }
        println!("{:?}", reader.headers().unwrap());
        println!("Finish Read");

        //array gets sorted for future use of adding new accounts by incrementing the ID number
        customer_arr.sort_by(|a,b| a.person.identification_number().cmp(&b.person.identification_number()));

        Ok(customer_arr)
    }

    //function to be called to manually created a new customer
    pub fn  create_new_customer(mut customer_arr: Vec<Customer>){

        let mut first_name = String::new();
        let mut last_name = String::new();
        let mut date_of_birth = String::new();
        let mut address = String::new();
        let mut email = String::new();
        let mut password = String::new();
        let mut phone_number = String::new();

        let mut new_checking:Option<checking::Checking> = None;
        let mut new_savings: Option<savings::Savings> = None;
        let mut new_credit: Option<credit::Credit> = None;
        
        let new_cust_id = customer_arr[customer_arr.len()-1].person.identification_number()+1;

        println!("Please include all fields");
        print!("First name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut first_name).expect("could not read input");
        first_name = first_name.trim().to_string();
        //println!("{}",first_name);

        print!("Last Name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut last_name).expect("could not read input");
        last_name = last_name.trim().to_string();
        //println!("{}",last_name);

        password = loop{

            print!("Enter Password: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("could not read input");

            let mut confirm_input = String::new();
            print!("Enter password again: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut confirm_input).expect("could not read input");

            input = input.trim().to_string();
            confirm_input = confirm_input.trim().to_string();

            if input == confirm_input{
                break input;
            }
            println!("Passwords did not match. try again.");

        };

        //add logic to enforce date format
        print!("Date of birth mm/dd/yyyy: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut date_of_birth).expect("could not read input");
        date_of_birth = date_of_birth.trim().to_string();
        
        print!("Address: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut address).expect("could not read input");
        address = address.trim().to_string();

        let phone_number_regex = Regex::new(r"^(?:\d{3}-){2}\d{4}$|^\(\d{3}\)\s*\d{3}-\d{4}$").unwrap();

        phone_number = loop{
            print!("Enter phone number xxx-xxx-xxxx : ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let phone_number_in = input.trim().to_string();

            //println!("{}",phone_number_in);

            if phone_number_regex.is_match(&phone_number_in){
                break phone_number_in;
            } else{
                println!("Phone number format incorrect, please enter number again.");
            }
        };

        let email_regex = Regex::new(r"^([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})$").unwrap();

        email = loop {
            print!("Enter email: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            email = input.trim().to_string();

            if email_regex.is_match(&email){
                break email;
            }else{
                println!("invalid email format, please enter email again.");
            }
        };

        //println!("{} and {}",phone_number, email);


        println!("Would you like to create a checking accouunt? y/n:");

        //consider turning into a function.
        new_checking = loop{
            let mut checking_decision = String::new();
            std::io::stdin().read_line(&mut checking_decision).expect("Error reading input");

            match checking_decision.trim() {
                "y" =>{
                    println!("Creating new Checking account...");

                    let new_checking_num = customer_arr.iter_mut().map(|customer| {
                        let mut acct = customer.account;
                        match &mut acct.checking() {
                            Some(checking) => checking.account_num(),
                            None => 1000,
                        }
                    }).max();
                    

                    //println!("Your Checking account number will be: {:?}",new_checking_num.unwrap());

                    print!("Enter initial balance: ");
                    std::io::stdout().flush().unwrap();
                    
                    let mut init_check_bal: f32 = 0.0;

                    loop {
                        let mut check_bal = String::new();
                        std::io::stdin().read_line(&mut check_bal).expect("Error reading input");

                        match check_bal.trim().parse::<f32>() {
                            Ok(bal) => {
                                format!("{:.2}",bal);
                                init_check_bal = bal;
                                break;
                            
                            },
                            Err(_) => {
                                println!("invalid input")
                            },
                        
                        }
                    }

                let new_checking_account = checking::Checking::new(new_checking_num.unwrap()+1, None, init_check_bal);

                break Some(new_checking_account);
            },
            "n" =>{
                println!("skipping creating Checking account");
                break None;
            },
            _ => println!("Invalid input, please enter y or n"),
            }
        };

        println!("Would you like to create a savings accouunt? y/n:");


        new_savings = loop{
            let mut savings_decision = String::new();
            std::io::stdin().read_line(&mut savings_decision).expect("Error reading input");

            match savings_decision.trim() {
                "y" =>{
                    println!("Creating new savings account...");

                    let new_savings_num = customer_arr.iter_mut().map(|customer| {
                        let mut acct = customer.account;
                        match &mut acct.savings() {
                            Some(savings) => savings.account_num(),
                            None => 2000,
                        }
                    }).max();
                    

                    //println!("Your savings account number will be: {:?}",new_savings_num.unwrap());

                    print!("Enter initial balance: ");
                    std::io::stdout().flush().unwrap();
                    
                    let mut init_check_bal: f32 = 0.0;

                    loop {
                        let mut check_bal = String::new();
                        std::io::stdin().read_line(&mut check_bal).expect("Error reading input");

                        match check_bal.trim().parse::<f32>() {
                            Ok(bal) => {
                                format!("{:.2}",bal);
                                init_check_bal = bal;
                                break;
                            
                            },
                            Err(_) => {
                                println!("invalid input")
                            },
                        
                        }
                    }

                let new_savings_account = savings::Savings::new(new_savings_num.unwrap()+1, None, init_check_bal);

                break Some(new_savings_account);
            },
            "n" =>{
                println!("skipping creating savings account");
                break None;
            },
            _ => println!("Invalid input, please enter y or n"),
            }
        };

        println!("Would you like to create a credit accouunt? y/n:");

        new_credit = loop{
            let mut credit_decision = String::new();
            std::io::stdin().read_line(&mut credit_decision).expect("Error reading input");

            match credit_decision.trim() {
                "y" =>{
                    println!("Creating new credit account...");

                    let new_credit_num = customer_arr.iter_mut().map(|customer| {
                        let mut acct = customer.account;
                        match &mut acct.credit() {
                            Some(credit) => credit.account_num(),
                            None => 3000,
                        }
                    }).max();
                    

                    //println!("Your credit account number will be: {:?}",new_credit_num.unwrap());

                    print!("Enter initial balance: ");
                    std::io::stdout().flush().unwrap();
                    
                    let mut init_check_bal: f32 = 0.0;

                    loop {
                        let mut check_bal = String::new();
                        std::io::stdin().read_line(&mut check_bal).expect("Error reading input");

                        match check_bal.trim().parse::<f32>() {
                            Ok(bal) => {
                                format!("{:.2}",bal);
                                init_check_bal = bal;
                                break;
                            
                            },
                            Err(_) => {
                                println!("invalid input")
                            },
                        
                        }
                    }

                    print!("Enter max credit amount: ");
                    std::io::stdout().flush().unwrap();
                    
                    let mut max_credit: f32 = 0.0;

                    loop {
                        let mut cred_max = String::new();
                        std::io::stdin().read_line(&mut cred_max).expect("Error reading input");

                        match cred_max.trim().parse::<f32>() {
                            Ok(bal) => {
                                format!("{:.2}",bal);
                                max_credit = bal;
                                break;
                            
                            },
                            Err(_) => {
                                println!("invalid input")
                            },
                        
                        }
                    }

                let new_credit_account = credit::Credit::new(new_credit_num.unwrap()+1, max_credit, None, init_check_bal);

                break Some(new_credit_account);
            },
            "n" =>{
                println!("skipping creating credit account");
                break None;
            },
            _ => println!("Invalid input, please enter y or n"),
            }
        };

        let new_person = person::Person::new(first_name, last_name, date_of_birth, new_cust_id, address, phone_number, email);
        
        let new_account = account::Account::new(new_cust_id, new_checking, new_savings, new_credit);

        let mut new_customer = customer::Customer::new(new_person, new_account, new_cust_id, password);

        new_customer.print_all_fields();

        customer_arr.push(new_customer);

    }

    pub fn print_all_fields(&mut self) {
        println!(
            "Name: {} {}",
            Self::person(self).first_name(),
            Self::person(self).last_name()
        );
        println!("DOB: {}", Self::person(self).date_of_birth());
        println!("Address: {}", Self::person(self).address());
        println!("ID: {}", Self::customer_num(self));
        println!("Phone Number: {}", Self::person(self).phone_number());
        println!("Email: {}", Self::person(self).email());
        println!("Password: {}", Self::password(&self));
        println!(
            "Savings Account Number: {}",
            match Self::account(self).savings(){
                Some(active_savings) => active_savings.account_num().to_string(),
                None => "No account!".to_string(),
            }
        );
        println!(
            "Checking Account Number: {}",
            match Self::account(self).checking(){
                Some(active_checking) => active_checking.account_num().to_string(),
                None => "No account!".to_string(),
            }
        );
        println!(
            "Credit Account Number: {}",
            match Self::account(self).credit(){
                Some(active_credit) => active_credit.account_num().to_string(),
                None => "No account!".to_string(),
            }
        );
    }

    pub fn print_all_balances(&mut self) {
        println!(
            "Savings Account bal: ${}",
            Self::account(self).savings().unwrap().balance()
        );
        println!(
            "Checking Account bal: ${}",
            Self::account(self).checking().unwrap().balance()
        );
        println!(
            "Credit Account bal: ${}",
            Self::account(self).credit().unwrap().balance()
        );
    }
}

//csv contents:
// Savings acct num, Last name, ID num, DOB, Checking acct Num, Credit acct Num, P#, Checking start bal, saving start bal, pass, credit start bal, address, First Name, Email, Credit max
