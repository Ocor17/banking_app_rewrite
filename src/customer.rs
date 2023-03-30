use crate::account::Account;
use crate::person::Person;
use argon2::{self, Argon2};
use csv;
use csv::ReaderBuilder;
use password_hash::{PasswordHasher, SaltString};
use rand::thread_rng;
//use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
//use std::io::prelude::*;
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
        //use of thread_rng should be checked.
        let salt = SaltString::generate(&mut thread_rng());
        let argon2 = Argon2::default();
        let new_password = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        new_password
    }

    pub fn csv_to_arr() -> Result<(), Box<dyn Error>> {
        let path = Path::new("Bank4.csv");
        let file = File::open(path)?;

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(file);

        for result in reader.records() {
            let record = result?;
            println!("{:?}", record);
        }
        println!("{:?}", reader.headers().unwrap());
        Ok(())
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
            Self::account(self).savings().unwrap().account_num()
        );
        println!(
            "Checking Account Number: {}",
            Self::account(self).checking().unwrap().account_num()
        );
        println!(
            "Credit Account Number: {}",
            Self::account(self).credit().unwrap().account_num()
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
