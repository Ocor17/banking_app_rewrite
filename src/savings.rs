#[derive(Debug, Clone, Copy, Default)]
pub struct Savings {
    account_num: i64,
    starting_balance: f32,
    balance: f32,
}
impl Savings {
    pub fn new(account_num: i64, starting_balance: f32, balance: f32) -> Self {
        Savings {
            account_num: account_num,
            starting_balance: starting_balance,
            balance: balance,
        }
    }

    pub fn account_num(&self) -> i64 {
        self.account_num
    }

    pub fn set_account_num(&mut self, account_num: i64) {
        self.account_num = account_num
    }

    pub fn starting_balance(&self) -> f32 {
        self.starting_balance
    }
    pub fn set_starting_balance(&mut self, starting_balance: f32) {
        self.starting_balance = starting_balance
    }
    pub fn balance(&self) -> f32 {
        self.balance
    }
    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance
    }
}
