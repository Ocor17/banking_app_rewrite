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
