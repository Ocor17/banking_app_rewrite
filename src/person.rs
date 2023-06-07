#[derive(Debug, Clone, Default)]

pub struct Person {
    first_name: String,
    last_name: String,
    date_of_birth: String,
    identification_number: i64,
    address: String,
    phone_number: String,
    email: String,
}

impl Person {
    pub fn new(
        first_name: String,
        last_name: String,
        date_of_birth: String,
        identification_number: i64,
        address: String,
        phone_number: String,
        email: String,
    ) -> Self {
        Person {
            first_name,
            last_name,
            date_of_birth,
            identification_number,
            address,
            phone_number,
            email,
        }
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name
    }

    pub fn date_of_birth(&self) -> String {
        self.date_of_birth.clone()
    }

    pub fn set_date_of_birth(&mut self, date_of_birth: String) {
        self.date_of_birth = date_of_birth
    }

    pub fn identification_number(&self) -> i64 {
        self.identification_number
    }

    pub fn set_identification_number(&mut self, identification_number: i64) {
        self.identification_number = identification_number
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address
    }

    pub fn phone_number(&self) -> String {
        self.phone_number.clone()
    }

    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number = phone_number
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email
    }
}
