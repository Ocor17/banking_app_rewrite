#[derive(Debug, Clone, Copy, Default)]
pub struct Credit {
    account_num: i64,
    max_credit: f32,
    starting_balance: Option<f32>,
    balance: f32,
}
impl Credit {
    pub fn new(
        account_num: i64,
        max_credit: f32,
        starting_balance: Option<f32>,
        balance: f32,
    ) -> Self {
        let starting_balance = starting_balance.unwrap_or(balance);

        Credit {
            account_num,
            max_credit,
            starting_balance: Some(starting_balance),
            balance,
        }
    }

    pub fn account_num(&self) -> i64 {
        self.account_num
    }
    pub fn set_account_num(&mut self, account_num: i64) {
        self.account_num = account_num;
    }
    pub fn max_credit(&self) -> f32 {
        self.max_credit
    }
    pub fn set_max_credit(&mut self, max_credit: f32) {
        self.max_credit = max_credit
    }
    pub fn starting_balance(&self) -> Option<f32> {
        self.starting_balance
    }

    //we're making the choice to not have a starting balance be overwritable
    pub fn set_starting_balance(&mut self, starting_balance: f32) {
        if self.starting_balance.is_none() {
            self.starting_balance = Some(starting_balance);
        }
    }
    pub fn balance(&self) -> f32 {
        self.balance
    }
    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_credit_initial_balance_given() {
        let credit = Credit::new(123, 5000.0, Some(100.0), 150.0);
        assert_eq!(credit.account_num(), 123);
        assert_eq!(credit.max_credit(), 5000.0);
        assert_eq!(credit.starting_balance(), Some(100.0));
        assert_eq!(credit.balance(), 150.0);
    }

    #[test]
    fn test_new_credit_initial_balance_none() {
        let credit = Credit::new(456, 10000.0, None, 200.0);
        assert_eq!(credit.account_num(), 456);
        assert_eq!(credit.max_credit(), 10000.0);
        assert_eq!(credit.starting_balance(), Some(200.0)); // Should be set to balance
        assert_eq!(credit.balance(), 200.0);
    }

    #[test]
    fn test_set_account_num_credit() {
        let mut credit = Credit::new(789, 2000.0, None, 50.0);
        credit.set_account_num(987);
        assert_eq!(credit.account_num(), 987);
    }

    #[test]
    fn test_set_max_credit() {
        let mut credit = Credit::new(101, 3000.0, Some(0.0), 0.0);
        credit.set_max_credit(3500.0);
        assert_eq!(credit.max_credit(), 3500.0);
    }

    #[test]
    fn test_set_balance_credit() {
        let mut credit = Credit::new(202, 4000.0, Some(100.0), 100.0);
        credit.set_balance(300.0);
        assert_eq!(credit.balance(), 300.0);
        // Starting balance should not be affected by set_balance in Credit
        assert_eq!(credit.starting_balance(), Some(100.0));

        let mut credit_none_start = Credit::new(203, 4500.0, None, 150.0);
        assert_eq!(credit_none_start.starting_balance(), Some(150.0)); // Set by new()
        credit_none_start.set_balance(175.0);
        assert_eq!(credit_none_start.balance(), 175.0);
        assert_eq!(credit_none_start.starting_balance(), Some(150.0)); // Still the initial value
    }

    #[test]
    fn test_set_starting_balance_credit() {
        let mut credit = Credit::default(); // starting_balance is None
        assert!(credit.starting_balance().is_none());

        credit.set_starting_balance(1000.0);
        assert_eq!(credit.starting_balance(), Some(1000.0));

        credit.set_starting_balance(2000.0); // Try to set again
        assert_eq!(credit.starting_balance(), Some(1000.0)); // Should not change
    }
}
