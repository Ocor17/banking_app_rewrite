#[derive(Debug, Clone, Copy, Default)]
pub struct Checking {
    account_num: i64,
    starting_balance: Option<f32>,
    balance: f32,
}

impl Checking {
    pub fn new(account_num: i64, starting_balance: Option<f32>, balance: f32) -> Self {
        let starting_balance = starting_balance.unwrap_or(balance);

        Checking {
            account_num,
            starting_balance: Some(starting_balance),
            balance,
        }
    }
    pub fn account_num(&self) -> i64 {
        self.account_num
    }
    pub fn set_account_num(&mut self, account_num: i64) {
        self.account_num = account_num
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
        if self.starting_balance.is_none() {
            self.starting_balance = Some(balance);
        }

        self.balance = balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_checking_initial_balance_given() {
        let checking = Checking::new(123, Some(100.0), 150.0);
        assert_eq!(checking.account_num(), 123);
        assert_eq!(checking.starting_balance(), Some(100.0));
        assert_eq!(checking.balance(), 150.0);
    }

    #[test]
    fn test_new_checking_initial_balance_none() {
        let checking = Checking::new(456, None, 200.0);
        assert_eq!(checking.account_num(), 456);
        assert_eq!(checking.starting_balance(), Some(200.0)); // Should be set to balance
        assert_eq!(checking.balance(), 200.0);
    }

    #[test]
    fn test_set_account_num_checking() {
        let mut checking = Checking::new(789, None, 50.0);
        checking.set_account_num(987);
        assert_eq!(checking.account_num(), 987);
    }

    #[test]
    fn test_set_balance_checking() {
        // Test case 1: starting_balance was initially None
        let mut checking1 = Checking::new(101, None, 250.0);
        // starting_balance should be Some(250.0) due to new()
        assert_eq!(checking1.starting_balance(), Some(250.0));
        checking1.set_balance(300.0);
        assert_eq!(checking1.balance(), 300.0);
        // starting_balance should still be Some(250.0) because it was set by new()
        // and set_balance only sets starting_balance if it's *still* None.
        assert_eq!(checking1.starting_balance(), Some(250.0));

        // Test case 2: starting_balance was Some
        let mut checking2 = Checking::new(102, Some(500.0), 550.0);
        assert_eq!(checking2.starting_balance(), Some(500.0));
        checking2.set_balance(600.0);
        assert_eq!(checking2.balance(), 600.0);
        assert_eq!(checking2.starting_balance(), Some(500.0)); // Should not change

        // Test case 3: test the specific logic in set_balance for setting starting_balance
        // This requires `starting_balance` to be `None` when `set_balance` is called.
        // The current `Checking::new` makes this tricky as it always sets `starting_balance`.
        // We'll simulate it by creating a default and then setting.
        let mut checking3 = Checking::default(); // starting_balance is None
        assert!(checking3.starting_balance().is_none());
        checking3.set_balance(100.0);
        assert_eq!(checking3.balance(), 100.0);
        assert_eq!(checking3.starting_balance(), Some(100.0)); // Now it should be set
    }

    #[test]
    fn test_set_starting_balance_checking() {
        let mut checking = Checking::default(); // starting_balance is None
        assert!(checking.starting_balance().is_none());

        checking.set_starting_balance(1000.0);
        assert_eq!(checking.starting_balance(), Some(1000.0));

        checking.set_starting_balance(2000.0); // Try to set again
        assert_eq!(checking.starting_balance(), Some(1000.0)); // Should not change
    }
}
