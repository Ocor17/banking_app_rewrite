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
        Credit {

            account_num: account_num,
            max_credit: max_credit,
            starting_balance: starting_balance,
            balance: balance,
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
        if self.starting_balance == None {
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
