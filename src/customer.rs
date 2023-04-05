use crate::account::{Account, self};
use crate::{checking, savings, credit};
use crate::person::{Person, self};
use argon2::{self, Argon2};
use csv;
use csv::ReaderBuilder;
use password_hash::{PasswordHasher, SaltString};
use rand::thread_rng;
//use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io;
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
        Ok(customer_arr)
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
