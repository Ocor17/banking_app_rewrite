use crate::customer::Customer;
use chrono::{DateTime, Local};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub struct BankStatement {}

impl BankStatement {
    pub fn create_bank_statement(current_account: &mut Customer) {
        let now: DateTime<Local> = Local::now();
        let formatted_date = now.format("%Y-%m-%d");
        let file_name = current_account.person().first_name() + "-BankStatement.txt";

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)
            .unwrap();
        let mut writer = BufWriter::new(file);
        //5 tabs
        writer
            .write_all(
                format!(
                    "Miner Bank                    Checking Account Number: {}\n",
                    current_account.account().checking().unwrap().account_num()
                )
                .as_bytes(),
            )
            .unwrap();
        writer
            .write_all(
                format!(
                    "                              Savings Account Number:  {}\n",
                    current_account.account().savings().unwrap().account_num()
                )
                .as_bytes(),
            )
            .unwrap();
        writer
            .write_all(
                format!(
                    "                              Credit Account Number:   {}\n",
                    current_account.account().credit().unwrap().account_num()
                )
                .as_bytes(),
            )
            .unwrap();
        writer
            .write_all(
                format!(
                    "                              Statement Begin Date: {}\n",
                    formatted_date
                )
                .as_bytes(),
            )
            .unwrap();
        writer
            .write_all(
                format!(
                    "                              Statement End Date:   {}\n",
                    formatted_date
                )
                .as_bytes(),
            )
            .unwrap();

        writer
            .write_all(
                format!(
                    "{} {}\n",
                    current_account.person().first_name(),
                    current_account.person().last_name()
                )
                .as_bytes(),
            )
            .unwrap();

        writer
            .write_all(format!("{}\n\n", current_account.person().address()).as_bytes())
            .unwrap();

        writer.write_all(
            "___________________________________ Transactions ____________________________________\n"
                .as_bytes(),
        ).unwrap();

        //TODO! ADD LOGIC TO PRINT OUT THE TRANSACTION LOG

        writer.write_all(
            "Starting Balance                                                  Ending Balance\n"
                .as_bytes(),
        ).unwrap();

        writer
            .write_all(
                format!(
            "Checking: ${}                                                     Checking: ${}\n",
            current_account
                .account()
                .checking()
                .unwrap()
                .starting_balance().unwrap(),
            current_account.account().checking().unwrap().balance()
        )
                .as_bytes(),
            )
            .unwrap();

        writer
            .write_all(
                format!(
            "Savings:  ${}                                                     Savings:  ${}\n",
            current_account
                .account()
                .savings()
                .unwrap()
                .starting_balance().unwrap(),
            current_account.account().savings().unwrap().balance()
        )
                .as_bytes(),
            )
            .unwrap();

        writer
            .write_all(
                format!(
            "Credit:   ${}                                                    Credit:   ${}\n",
            current_account
                .account()
                .credit()
                .unwrap()
                .starting_balance().unwrap(),
            current_account.account().credit().unwrap().balance()
        )
                .as_bytes(),
            )
            .unwrap();

        writer.write_all(
                "____________________________________________________________________________________\n"
                    .as_bytes(),
            ).unwrap();
    }
}
