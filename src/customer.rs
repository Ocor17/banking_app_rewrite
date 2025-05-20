use crate::account::{self, Account};
use crate::person::{self, Person};
use crate::{checking, credit, customer, savings};
use argon2::{self, Argon2, PasswordHasher};
use csv;
use csv::ReaderBuilder;
use password_hash::{PasswordHash, PasswordVerifier, SaltString};
use rand::thread_rng;
//use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
//use std::io::prelude::*;
use regex::Regex;
use std::path::Path;

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
            person,
            account,
            customer_num,
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

    pub fn password_hashing(password: String) -> String {
        //use of thread_rng should be checked if this is the best way to get a rand for this implementation.
        let salt = SaltString::generate(&mut thread_rng());
        let argon2 = Argon2::default();
        let new_password = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        new_password
    }

    pub fn password_verify(&self, password: &str) -> bool {
        //let verfied_password = Argon2::default();

        let binding = self.password();
        let hash = PasswordHash::new(&binding).unwrap();

        if Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
        {
            println!("VERIFIED");
            return true;
        }
        println!("NOT VERIFIED");
        false
    }

    //currently takes some time to run this function, most likely due to how argon2 works
    pub fn csv_to_customer_arr_from_path(file_path: &Path) -> Result<Vec<Customer>, Box<dyn Error>> {
        let file = File::open(file_path)?;

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

            let person = person::Person::new(
                record[12].to_string(),
                record[1].to_string(),
                record[3].to_string(),
                record[2].parse()?, // Use ? for error propagation
                record[11].to_string(),
                record[6].to_string(),
                record[13].to_string(),
            );
            let checking = checking::Checking::new(
                record[4].parse()?, // Use ?
                None,
                record[7].parse()?, // Use ?
            );
            let savings =
                savings::Savings::new(record[0].parse()?, None, record[8].parse()?); // Use ?
            let credit = credit::Credit::new(
                record[5].parse()?, // Use ?
                record[14].parse()?, // Use ?
                None,
                record[10].parse()?, // Use ?
            );
            let account = account::Account::new(
                record[2].parse()?, // Use ?
                Some(checking),
                Some(savings),
                Some(credit),
            );
            let customer = Customer::new(
                person,
                account,
                record[2].parse()?, // Use ?
                record[9].to_string(),
            );

            println!("Creating customer: {} {}...", &record[12], &record[1]);

            customer_arr.push(customer);
            //let record = result?;
            //println!("{:?}", record);
        }
        println!("{:?}", reader.headers().unwrap());
        println!("Finish Read");

        //array gets sorted for future use of adding new accounts by incrementing the ID number
        customer_arr.sort_by(|a, b| {
            a.person
                .identification_number()
                .cmp(&b.person.identification_number())
        });

        Ok(customer_arr)
    }

    pub fn csv_to_customer_arr() -> Result<Vec<Customer>, Box<dyn Error>> {
        let mut input = String::new();
        println!("Input csv filename:");
        io::stdin().read_line(&mut input)?;
        let path = Path::new(input.trim());
        Self::csv_to_customer_arr_from_path(path)
    }

    //function to be called to manually created a new customer
    //need to add checks to ensure inital balance is not less than 0
    pub fn create_new_customer(customer_arr: &mut Vec<Customer>) {
        let mut first_name = String::new();
        let mut last_name = String::new();
        let mut date_of_birth = String::new();
        let mut address = String::new();
        let mut email = String::new();
        let mut password = String::new();
        let mut phone_number = String::new();

        let mut new_checking: Option<checking::Checking> = None;
        let mut new_savings: Option<savings::Savings> = None;
        let mut new_credit: Option<credit::Credit> = None;

        let new_cust_id = customer_arr[customer_arr.len() - 1]
            .person
            .identification_number()
            + 1;

        println!("Please include all fields");
        print!("First name: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut first_name)
            .expect("could not read input");
        first_name = first_name.trim().to_string();
        //println!("{}",first_name);

        print!("Last Name: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut last_name)
            .expect("could not read input");
        last_name = last_name.trim().to_string();
        //println!("{}",last_name);

        password = loop {
            print!("Enter Password: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("could not read input");

            let mut confirm_input = String::new();
            print!("Enter password again: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut confirm_input)
                .expect("could not read input");

            input = input.trim().to_string();
            confirm_input = confirm_input.trim().to_string();

            if input == confirm_input {
                break input;
            }
            println!("Passwords did not match. try again.");
        };

        //regex enforces mm/dd/yyyy format
        let dob_regex =
            Regex::new(r#"^(0[1-9]|1[0-2])/(0[1-9]|[1-2][0-9]|3[0-1])/\d{4}$"#).unwrap();

        date_of_birth = loop {
            print!("Date of birth mm/dd/yyyy: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut date_of_birth)
                .expect("could not read input");
            date_of_birth = date_of_birth.trim().to_string();

            if dob_regex.is_match(&date_of_birth) {
                break date_of_birth;
            }
            println!("date of birth format incorrect!");
        };

        print!("Address: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut address)
            .expect("could not read input");
        address = address.trim().to_string();

        //regex enforces xxx-xxx-xxxx format
        let phone_number_regex =
            Regex::new(r"^(?:\d{3}-){2}\d{4}$|^\(\d{3}\)\s*\d{3}-\d{4}$").unwrap();

        phone_number = loop {
            print!("Enter phone number xxx-xxx-xxxx : ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let phone_number_in = input.trim().to_string();

            //println!("{}",phone_number_in);

            if phone_number_regex.is_match(&phone_number_in) {
                break phone_number_in;
            }
            println!("Phone number format incorrect, please enter number again.");
        };

        let email_regex =
            Regex::new(r"^([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})$").unwrap();

        email = loop {
            print!("Enter email: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            email = input.trim().to_string();

            if email_regex.is_match(&email) {
                break email;
            } else {
                println!("invalid email format, please enter email again.");
            }
        };

        //println!("{} and {}",phone_number, email);

        println!("Would you like to create a checking accouunt? y/n:");

        //consider turning into a function.
        new_checking = loop {
            let mut checking_decision = String::new();
            std::io::stdin()
                .read_line(&mut checking_decision)
                .expect("Error reading input");

            match checking_decision.trim() {
                "y" => {
                    println!("Creating new Checking account...");

                    let new_checking_num = customer_arr
                        .iter_mut()
                        .map(|customer| {
                            let mut acct = customer.account;
                            match &mut acct.checking() {
                                Some(checking) => checking.account_num(),
                                None => 1000,
                            }
                        })
                        .max();

                    //println!("Your Checking account number will be: {:?}",new_checking_num.unwrap());

                    print!("Enter initial balance: ");
                    std::io::stdout().flush().unwrap();

                    let mut init_check_bal: f32 = 0.0;

                    loop {
                        let mut check_bal = String::new();
                        std::io::stdin()
                            .read_line(&mut check_bal)
                            .expect("Error reading input");

                        match check_bal.trim().parse::<f32>() {
                            Ok(bal) => {
                                format!("{:.2}", bal);
                                init_check_bal = bal;
                                break;
                            }
                            Err(_) => {
                                println!("invalid input")
                            }
                        }
                    }

                    let new_checking_account = checking::Checking::new(
                        new_checking_num.unwrap() + 1,
                        None,
                        init_check_bal,
                    );

                    break Some(new_checking_account);
                }
                "n" => {
                    println!("skipping creating Checking account");
                    break None;
                }
                _ => println!("Invalid input, please enter y or n"),
            }
        };

        println!("Would you like to create a savings accouunt? y/n:");

        new_savings = loop {
            let mut savings_decision = String::new();
            std::io::stdin()
                .read_line(&mut savings_decision)
                .expect("Error reading input");

            match savings_decision.trim() {
                "y" => {
                    println!("Creating new savings account...");

                    let new_savings_num = customer_arr
                        .iter_mut()
                        .map(|customer| {
                            let mut acct = customer.account;
                            match &mut acct.savings() {
                                Some(savings) => savings.account_num(),
                                None => 2000,
                            }
                        })
                        .max();

                    //println!("Your savings account number will be: {:?}",new_savings_num.unwrap());

                    print!("Enter initial balance: ");
                    std::io::stdout().flush().unwrap();

                    let mut init_check_bal: f32 = 0.0;

                    loop {
                        let mut check_bal = String::new();
                        std::io::stdin()
                            .read_line(&mut check_bal)
                            .expect("Error reading input");

                        match check_bal.trim().parse::<f32>() {
                            Ok(bal) => {
                                format!("{:.2}", bal);
                                init_check_bal = bal;
                                break;
                            }
                            Err(_) => {
                                println!("invalid input")
                            }
                        }
                    }

                    let new_savings_account =
                        savings::Savings::new(new_savings_num.unwrap() + 1, None, init_check_bal);

                    break Some(new_savings_account);
                }
                "n" => {
                    println!("skipping creating savings account");
                    break None;
                }
                _ => println!("Invalid input, please enter y or n"),
            }
        };

        println!("Would you like to create a credit accouunt? y/n:");

        new_credit = loop {
            let mut credit_decision = String::new();
            std::io::stdin()
                .read_line(&mut credit_decision)
                .expect("Error reading input");

            match credit_decision.trim() {
                "y" => {
                    println!("Creating new credit account...");

                    let new_credit_num = customer_arr
                        .iter_mut()
                        .map(|customer| {
                            let mut acct = customer.account;
                            match &mut acct.credit() {
                                Some(credit) => credit.account_num(),
                                None => 3000,
                            }
                        })
                        .max();

                    //println!("Your credit account number will be: {:?}",new_credit_num.unwrap());

                    print!("Enter initial balance: ");
                    std::io::stdout().flush().unwrap();

                    let mut init_check_bal: f32 = 0.0;

                    loop {
                        let mut check_bal = String::new();
                        std::io::stdin()
                            .read_line(&mut check_bal)
                            .expect("Error reading input");

                        match check_bal.trim().parse::<f32>() {
                            Ok(bal) => {
                                format!("{:.2}", bal);
                                init_check_bal = bal;
                                break;
                            }
                            Err(_) => {
                                println!("invalid input")
                            }
                        }
                    }

                    print!("Enter max credit amount: ");
                    std::io::stdout().flush().unwrap();

                    let mut max_credit: f32 = 0.0;

                    loop {
                        let mut cred_max = String::new();
                        std::io::stdin()
                            .read_line(&mut cred_max)
                            .expect("Error reading input");

                        match cred_max.trim().parse::<f32>() {
                            Ok(bal) => {
                                format!("{:.2}", bal);
                                max_credit = bal;
                                break;
                            }
                            Err(_) => {
                                println!("invalid input")
                            }
                        }
                    }

                    let new_credit_account = credit::Credit::new(
                        new_credit_num.unwrap() + 1,
                        max_credit,
                        None,
                        init_check_bal,
                    );

                    break Some(new_credit_account);
                }
                "n" => {
                    println!("skipping creating credit account");
                    break None;
                }
                _ => println!("Invalid input, please enter y or n"),
            }
        };

        let new_person = person::Person::new(
            first_name,
            last_name,
            date_of_birth,
            new_cust_id,
            address,
            phone_number,
            email,
        );

        let new_account = account::Account::new(new_cust_id, new_checking, new_savings, new_credit);

        let mut new_customer =
            customer::Customer::new(new_person, new_account, new_cust_id, password);

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
        println!("Password: {}", Self::password(self));
        println!(
            "Savings Account Number: {}",
            match Self::account(self).savings() {
                Some(active_savings) => active_savings.account_num().to_string(),
                None => "No account!".to_string(),
            }
        );
        println!(
            "Checking Account Number: {}",
            match Self::account(self).checking() {
                Some(active_checking) => active_checking.account_num().to_string(),
                None => "No account!".to_string(),
            }
        );
        println!(
            "Credit Account Number: {}",
            match Self::account(self).credit() {
                Some(active_credit) => active_credit.account_num().to_string(),
                None => "No account!".to_string(),
            }
        );
    }

    pub fn print_all_balances(&mut self) {
        println!("Printing all account balances...");

        println!(
            "Savings Account bal: ${}",
            match Self::account(self).savings() {
                Some(active_savings) => active_savings.balance().to_string(),
                None => "No account!".to_string(),
            }
        );
        println!(
            "Checking Account bal: ${}",
            match Self::account(self).checking() {
                Some(active_checking) => active_checking.balance().to_string(),
                None => "No account!".to_string(),
            }
        );
        println!(
            "Credit Account bal: ${}",
            match Self::account(self).credit() {
                Some(active_credit) => active_credit.balance().to_string(),
                None => "No account!".to_string(),
            }
        );
    }
}

//csv contents:
// Savings acct num, Last name, ID num, DOB, Checking acct Num, Credit acct Num, P#, Checking start bal, saving start bal, pass, credit start bal, address, First Name, Email, Credit max

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::Account;
    use crate::person::Person;
    use std::path::Path;

    fn create_test_person() -> Person {
        Person::new(
            "Test".to_string(),
            "User".to_string(),
            "01/01/2000".to_string(),
            999,
            "1 Test Lane".to_string(),
            "555-TEST".to_string(),
            "test@example.com".to_string(),
        )
    }

    fn create_test_account() -> Account {
        Account::new(999, None, None, None)
    }

    #[test]
    fn test_new_customer() {
        let person = create_test_person();
        let account = create_test_account();
        let mut customer = Customer::new(person.clone(), account.clone(), 777, "password123".to_string());

        assert_eq!(customer.customer_num(), 777);
        assert_eq!(customer.person().first_name(), "Test"); // Check a field from Person
        assert_eq!(customer.account().account_num(), 999); // Check a field from Account
    }

    #[test]
    fn test_set_customer_num() {
        let mut customer = Customer::new(create_test_person(), create_test_account(), 777, "pw".to_string());
        customer.set_customer_num(888);
        assert_eq!(customer.customer_num(), 888);
    }

    #[test]
    fn test_set_person() {
        let mut customer = Customer::new(create_test_person(), create_test_account(), 777, "pw".to_string());
        let mut new_person = create_test_person();
        new_person.set_first_name("Updated".to_string());
        customer.set_person(new_person.clone());
        assert_eq!(customer.person().first_name(), "Updated");
    }

    #[test]
    fn test_set_account() {
        let mut customer = Customer::new(create_test_person(), create_test_account(), 777, "pw".to_string());
        let mut new_account = create_test_account();
        new_account.set_account_num(1001);
        customer.set_account(new_account.clone());
        assert_eq!(customer.account().account_num(), 1001);
    }

    #[test]
    fn test_password_hashing_and_verify() {
        let sample_password = "securePassword!";
        // Hashing is done in Customer::new or set_password
        let customer = Customer::new(create_test_person(), create_test_account(), 777, sample_password.to_string());

        assert!(customer.password_verify(sample_password));
        assert!(!customer.password_verify("wrongPassword"));
    }

    #[test]
    fn test_set_password() {
        let mut customer = Customer::new(create_test_person(), create_test_account(), 777, "oldPassword".to_string());
        
        let new_password = "newSecurePassword123";
        customer.set_password(new_password.to_string());

        assert!(customer.password_verify(new_password));
        assert!(!customer.password_verify("oldPassword"));
    }

    #[test]
    fn test_csv_to_customer_arr_success() {
        let path = Path::new("src/test_data/mock_customers.csv");
        let result = Customer::csv_to_customer_arr_from_path(path);
        
        assert!(result.is_ok());
        let mut customers = result.unwrap(); // Make customers mutable
        assert_eq!(customers.len(), 2);

        // Check some data from the first customer (Smith)
        // Use iter_mut() to get a mutable reference if needed by person() or account()
        let smith = customers.iter_mut().find(|c| c.customer_num() == 101).unwrap();
        assert_eq!(smith.person().first_name(), "John");
        assert_eq!(smith.person().last_name(), "Smith");
        assert!(smith.account().checking().is_some());
        assert_eq!(smith.account().checking().as_ref().unwrap().balance(), 1000.00);
        assert_eq!(smith.account().savings().as_ref().unwrap().balance(), 2000.00);
        assert_eq!(smith.account().credit().as_ref().unwrap().balance(), 50.00);
         assert!(smith.password_verify("pass123"));
    }

    #[test]
    fn test_csv_to_customer_arr_file_not_found() {
        let path = Path::new("src/test_data/non_existent_file.csv");
        let result = Customer::csv_to_customer_arr_from_path(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_csv_to_customer_arr_malformed_data() {
        let path = Path::new("src/test_data/mock_customers_malformed.csv");
        let result = Customer::csv_to_customer_arr_from_path(path);
        assert!(result.is_err()); // Expecting an error due to "NOT_A_FLOAT"
    }
}
