
pub mod customer;
use std::io;

pub struct Main_Menu{
    customers:Vec<customers>
}

impl Main_Menu{
    pub fn new(customers:Vec<customers>) -> Self {
        Main_Menu{
            customers:customers,
        }

    }

    pub fn bank_manager(acc:Vec<customers>){
        
    }

    pub fn balance_sub_menu(acc:Customer, from_where:String){

        let  mut selection = String::new();
        while true{
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


}