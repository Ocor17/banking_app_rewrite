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
            acct_num,
            active: true,
            checking,
            savings,
            credit,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{checking::Checking, credit::Credit, savings::Savings};

    #[test]
    fn test_new_account_with_none_accounts_and_getters() {
        let mut account = Account::new(12345, None, None, None);
        assert_eq!(account.account_num(), 12345);
        assert_eq!(account.active(), true);
        assert!(account.checking().is_none());
        assert!(account.savings().is_none());
        assert!(account.credit().is_none());
    }

    #[test]
    fn test_new_account_with_some_accounts() {
        let checking_acc = Checking::new(1, Some(100.0), 100.0);
        let savings_acc = Savings::new(2, Some(200.0), 200.0);
        let credit_acc = Credit::new(3, 1000.0, Some(0.0), 0.0);
        let mut account = Account::new(
            67890,
            Some(checking_acc),
            Some(savings_acc),
            Some(credit_acc),
        );

        assert!(account.checking().is_some());
        assert!(account.savings().is_some());
        assert!(account.credit().is_some());
    }

    #[test]
    fn test_set_account_num_and_active() {
        let mut account = Account::new(111, None, None, None);
        account.set_account_num(222);
        assert_eq!(account.account_num(), 222);

        account.set_active(false);
        assert_eq!(account.active(), false);
    }

    #[test]
    fn test_set_checking_savings_credit() {
        let mut account = Account::new(333, None, None, None);

        let checking_acc = Checking::new(10, Some(50.0), 50.0);
        account.set_checking(checking_acc);
        assert!(account.checking().is_some());
        assert_eq!(account.checking().unwrap().balance(), 50.0);

        let savings_acc = Savings::new(20, Some(150.0), 150.0);
        account.set_savings(savings_acc);
        assert!(account.savings().is_some());
        assert_eq!(account.savings().unwrap().balance(), 150.0);

        let credit_acc = Credit::new(30, 2000.0, Some(0.0), 0.0);
        account.set_credit(credit_acc);
        assert!(account.credit().is_some());
        assert_eq!(account.credit().unwrap().balance(), 0.0);
    }

    #[test]
    fn test_get_and_modify_mutable_checking() {
        let checking_acc = Checking::new(101, Some(500.0), 500.0);
        let mut account = Account::new(444, Some(checking_acc), None, None);

        assert!(account.checking().is_some());
        if let Some(c_ref) = account.checking().as_mut() {
            c_ref.set_balance(550.0);
        }
        assert_eq!(account.checking().unwrap().balance(), 550.0);
    }

    #[test]
    fn test_get_and_modify_mutable_savings() {
        let savings_acc = Savings::new(202, Some(1000.0), 1000.0);
        let mut account = Account::new(555, None, Some(savings_acc), None);

        assert!(account.savings().is_some());
        if let Some(s_ref) = account.savings().as_mut() {
            s_ref.set_balance(1100.0);
        }
        assert_eq!(account.savings().unwrap().balance(), 1100.0);
    }

    #[test]
    fn test_get_and_modify_mutable_credit() {
        let credit_acc = Credit::new(303, 5000.0, Some(200.0), 200.0);
        let mut account = Account::new(666, None, None, Some(credit_acc));

        assert!(account.credit().is_some());
        if let Some(cr_ref) = account.credit().as_mut() {
            cr_ref.set_balance(250.0);
            cr_ref.set_max_credit(5500.0);
        }
        assert_eq!(account.credit().unwrap().balance(), 250.0);
        assert_eq!(account.credit().unwrap().max_credit(), 5500.0);
    }

    #[test]
    fn test_get_mutable_references_when_none() {
        let mut account = Account::new(777, None, None, None);

        assert!(account.checking().is_none());
        assert!(account.savings().is_none());
        assert!(account.credit().is_none());

        // Try to modify, should not panic and remain None
        if let Some(c_ref) = account.checking().as_mut() {
            c_ref.set_balance(10.0); // This block should not be entered
        }
        assert!(account.checking().is_none()); // Still None

        if let Some(s_ref) = account.savings().as_mut() {
            s_ref.set_balance(20.0); // This block should not be entered
        }
        assert!(account.savings().is_none()); // Still None

        if let Some(cr_ref) = account.credit().as_mut() {
            cr_ref.set_balance(30.0); // This block should not be entered
        }
        assert!(account.credit().is_none()); // Still None
    }
}
