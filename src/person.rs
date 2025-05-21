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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_person_and_getters() {
        let person = Person::new(
            "John".to_string(),
            "Doe".to_string(),
            "1990-01-01".to_string(),
            1234567890,
            "123 Main St, Anytown, USA".to_string(),
            "555-123-4567".to_string(),
            "john.doe@example.com".to_string(),
        );

        assert_eq!(person.first_name(), "John");
        assert_eq!(person.last_name(), "Doe");
        assert_eq!(person.date_of_birth(), "1990-01-01");
        assert_eq!(person.identification_number(), 1234567890);
        assert_eq!(person.address(), "123 Main St, Anytown, USA");
        assert_eq!(person.phone_number(), "555-123-4567");
        assert_eq!(person.email(), "john.doe@example.com");
    }

    #[test]
    fn test_set_first_name() {
        let mut person = Person::default();
        person.set_first_name("Jane".to_string());
        assert_eq!(person.first_name(), "Jane");
    }

    #[test]
    fn test_set_last_name() {
        let mut person = Person::default();
        person.set_last_name("Doe".to_string());
        assert_eq!(person.last_name(), "Doe");
    }

    #[test]
    fn test_set_date_of_birth() {
        let mut person = Person::default();
        person.set_date_of_birth("2000-12-31".to_string());
        assert_eq!(person.date_of_birth(), "2000-12-31");
    }

    #[test]
    fn test_set_identification_number() {
        let mut person = Person::default();
        person.set_identification_number(9876543210);
        assert_eq!(person.identification_number(), 9876543210);
    }

    #[test]
    fn test_set_address() {
        let mut person = Person::default();
        person.set_address("456 Oak Ave, Otherville, USA".to_string());
        assert_eq!(person.address(), "456 Oak Ave, Otherville, USA");
    }

    #[test]
    fn test_set_phone_number() {
        let mut person = Person::default();
        person.set_phone_number("555-987-6543".to_string());
        assert_eq!(person.phone_number(), "555-987-6543");
    }

    #[test]
    fn test_set_email() {
        let mut person = Person::default();
        person.set_email("jane.doe@example.com".to_string());
        assert_eq!(person.email(), "jane.doe@example.com");
    }
}
