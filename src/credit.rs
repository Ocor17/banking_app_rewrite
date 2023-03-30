#[derive(Debug, Clone, Copy, Default)]
pub struct Credit {
    id_num: i64,
    account_num: i64,
    max_credit: f32,
    starting_balance: f32,
    balance: f32,
}
impl Credit {
    pub fn new(
        id_num: i64,
        account_num: i64,
        max_credit: f32,
        starting_balance: f32,
        balance: f32,
    ) -> Self {
        Credit {
            id_num: id_num,
            account_num: account_num,
            max_credit: max_credit,
            starting_balance: starting_balance,
            balance: balance,
        }
    }

    pub fn id_num(&self) -> i64 {
        self.id_num
    }
    pub fn set_id_num(&mut self, id_num: i64) {
        self.id_num = id_num;
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
