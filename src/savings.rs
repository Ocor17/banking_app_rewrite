#[derive(Debug, Clone, Copy, Default)]
pub struct Savings {
    account_num: i64,
    starting_balance: Option<f32>,
    balance: f32,
}
impl Savings {
    pub fn new(account_num: i64, starting_balance: Option<f32>, balance: f32) -> Self {
        let starting_balance = starting_balance.unwrap_or(balance);

        Savings {
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
        self.balance = balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_savings_initial_balance_given() {
        let savings = Savings::new(123, Some(100.0), 150.0);
        assert_eq!(savings.account_num(), 123);
        assert_eq!(savings.starting_balance(), Some(100.0));
        assert_eq!(savings.balance(), 150.0);
    }

    #[test]
    fn test_new_savings_initial_balance_none() {
        let savings = Savings::new(456, None, 200.0);
        assert_eq!(savings.account_num(), 456);
        assert_eq!(savings.starting_balance(), Some(200.0)); // Should be set to balance
        assert_eq!(savings.balance(), 200.0);
    }

    #[test]
    fn test_set_account_num_savings() {
        let mut savings = Savings::new(789, None, 50.0);
        savings.set_account_num(987);
        assert_eq!(savings.account_num(), 987);
    }

    #[test]
    fn test_set_balance_savings() {
        let mut savings = Savings::new(101, Some(250.0), 250.0);
        savings.set_balance(300.0);
        assert_eq!(savings.balance(), 300.0);
        // Starting balance should not be affected by set_balance in Savings
        assert_eq!(savings.starting_balance(), Some(250.0));

        let mut savings_none_start = Savings::new(102, None, 400.0);
        assert_eq!(savings_none_start.starting_balance(), Some(400.0)); // Set by new()
        savings_none_start.set_balance(450.0);
        assert_eq!(savings_none_start.balance(), 450.0);
        assert_eq!(savings_none_start.starting_balance(), Some(400.0)); // Still the initial value
    }

    #[test]
    fn test_set_starting_balance_savings() {
        let mut savings = Savings::default(); // starting_balance is None
        assert!(savings.starting_balance().is_none());

        savings.set_starting_balance(1000.0);
        assert_eq!(savings.starting_balance(), Some(1000.0));

        savings.set_starting_balance(2000.0); // Try to set again
        assert_eq!(savings.starting_balance(), Some(1000.0)); // Should not change
    }
}
