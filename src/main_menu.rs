
use std::{io::{self, Write, Read}, borrow::BorrowMut};
use crate::{customer::{Customer, self}, main_menu};

#[derive(Debug, Clone, Default)]
pub struct MainMenu{
    customers:Vec<Customer>
}

impl MainMenu{
    pub fn new(customers:Vec<Customer>) -> Self {
        MainMenu{
            customers:customers,
        }

    }

    pub fn bank_manager(accounts:Vec<Customer>){
        
    }

    pub fn main_menu(mut accounts:Vec<Customer>) ->  Vec<Customer>{

        let mut selection = String::new();
        let mut first_name = String::new();
        let mut last_name = String::new();
        let mut payer_id_input =String::new();
        let mut payer_index:i64 = -1;
        let mut payer_id:i64 = -1;

        let mut  login_selector = String::new();
        let mut password = String::new();
        let mut current_account = None;


        loop{
            
            //add logic to escape when inside  one of the options
            println!("Choose how to login by entering 1 or 2:");
            println!("1. ID number e.g. 00");
            println!("2. First name and Last name");
            println!("3. Exit");

            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut selection).expect("Error reading input");


            match selection.trim(){
                "1" =>{
                    loop {
                        println!("Enter ID number in Full e.g. 00:");
                        std::io::stdout().flush().unwrap();
                        std::io::stdin().read_line(&mut payer_id_input).expect("failed to read");

                        match payer_id_input.trim().parse::<i64>(){
                            Ok(parsed_id) =>{

                                current_account = loop {
                                    match accounts.iter_mut().find(|p| p.customer_num() == parsed_id) {
                                        Some(single_account)=>{
                                            break Some(single_account);
                                        },
                                        None =>{
                                            println!("No such account exists");
                                            break None;
                                        },
                                    }
                                    
                                };


                                //payer_id = parsed_id;
                                if current_account.is_some(){
                                    break;
                                }
                                payer_id_input.clear();

                            }
                            Err(_) =>{
                                println!("Invalid input, not a integer");
                                std::io::stdout().flush().unwrap();
                                payer_id_input.clear();
                            }
                        }
                    }
                    break;
                },
                "2" => {
                    loop{
                    println!("Enter First name");
                    std::io::stdout().flush().unwrap();
                    
                    std::io::stdin().read_line(&mut first_name).expect("Error reading input.");

                    println!("Enter Last name");
                    std::io::stdout().flush().unwrap();
                    
                    std::io::stdin().read_line(&mut last_name).expect("Error  reading input.");


                    //Using for loop in this section to avoid borrowing issues from presumbly checking too fields of an element
                    current_account ={

                        let mut result: Option<&mut Customer> = None;

                        for accts in accounts.iter_mut(){
                            if accts.person().first_name() == first_name.trim() && accts.person().last_name() == last_name.trim(){
                                
                                result =  Some(accts);
                                break;

                            }
                        }
                        result
                        
                    };

                    if current_account.is_some(){
                        break;
                    }
                    println!("No such person exists");
                    first_name.clear();
                    last_name.clear();
                    }
                    break;
                },
                "3" => {
                    println!("Goodbye!");
                    break;
                },
                _ => {
                    println!("Invalid input!");
                    payer_id_input.clear();
                },
                
            }
        }
        //println!("ID:{} fn:{} ln:{}", payer_id,first_name,last_name);

        let current_account  = current_account.unwrap();
        
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
        std::io::stdin().read_line(&mut password).expect("Issue reading input");

        let input_hash = customer::Customer::password_hashing(password);

        if current_account.password() != input_hash{
            println!("Wrong password! Exiting");
            return accounts;
        }
        println!("Correct password... Logging in...");

        loop {

            selection.clear();

            println!("Select what you want to do by typing only the number");
            println!("1. Show balance");
            println!("2. Transfer moneyt");
            println!("3. Deposit money");
            println!("4. Withdraw money");
            println!("5. Pay someone");
            println!("6. Exit");

            std::io::stdin().read_line(&mut selection);

            match selection.trim() {
                "1" =>{
                    Self::balance_sub_menu(current_account, "".to_string());
                    break;
                },
                "2" =>{
                    Self::transfer_sub_menu();
                    break;
                },
                "3" =>{
                    Self::deposit_sub_menu();
                    break;
                },
                "4" =>{
                    Self::withdraw_sub_menu();
                    break;
                },
                "5" =>{
                    Self::pay_someone_sub_menu();
                    break;
                },
                "6" =>{
                    println!("Returning to home page...");
                    Self::new_balance_sheet();
                    return accounts;
                },
                _ => println!("-----------Invalid input!-----------")
            }

        }

        
        accounts

    }

    pub fn balance_sub_menu(acc:&mut Customer, from_where:String){

        let  mut selection = String::new();
        loop{
            if(selection == ""){
                println!("Choose account type:
                1. Checking
                2. Savings
                3. Credit
                4. Exit");
                
                std::io::stdin().read_line(&mut selection).expect("Failed to read input");
            }   

        }
    }

    pub fn transfer_sub_menu(){

    }

    pub fn deposit_sub_menu(){

    }

    pub fn withdraw_sub_menu(){

    }

    pub fn pay_someone_sub_menu(){

    }

    pub fn transaction_log(){

    }

    pub fn new_balance_sheet(){

    }

    pub fn print_all_fields(){

    }

    pub fn  print_balance(){

    }



}