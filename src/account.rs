use crate::checking::Checking;
use crate::credit::Credit;
use crate::savings::Savings;

#[derive(Debug, Clone, Copy, Default)]
//consider moving account number to "Customer"
pub struct Account {
    acct_num: i64,
    //consider renaming exists to active
    active: bool,
    checking: Option<Checking>,
    savings: Option<Savings>,
    credit: Option<Credit>,
}
impl Account {
    pub fn new(
        acct_num: i64,
        checking: Option<Checking>,
        savings: Option<Savings>,
        credit: Option<Credit>,
    ) -> Self {
        Account {
            acct_num: acct_num,
            active: true,
            checking: checking,
            savings: savings,
            credit: credit,
        }
    }

    pub fn account_num(&self) -> i64 {
        self.acct_num
    }

    pub fn set_account_num(&mut self, account_num: i64) {
        self.acct_num = account_num
    }
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active
    }

    pub fn checking(&mut self) -> &mut Option<Checking> {
        match &mut self.checking {
            Some(_checking) => &mut self.checking,
            None => {
                //println!("No Checking Account error");
                &mut self.checking
            }
        }
        //&mut self.checking
    }
    pub fn set_checking(&mut self, checking: Checking) {
        self.checking = Some(checking);
    }
    pub fn savings(&mut self) -> &mut Option<Savings> {
        match &mut self.savings {
            Some(_saving) => &mut self.savings,
            None => {
                //println!("No Savings Account error");
                &mut self.savings
            }
        }
    }
    pub fn set_savings(&mut self, savings: Savings) {
        self.savings = Some(savings)
    }
    pub fn credit(&mut self) -> &mut Option<Credit> {
        match &mut self.credit {
            Some(_credit) => &mut self.credit,
            None => {
                //println!("No Creidt Account error");
                &mut self.credit
            }
        }
    }
    pub fn set_credit(&mut self, credit: Credit) {
        self.credit = Some(credit)
    }
}
