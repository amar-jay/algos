#[derive(Default)]
#[allow(unused)]
struct Builder {
    name: String,
    age: u8,
    address: String,
    phone: String,
}

#[allow(unused)]
struct Contact {
    name: String,
    age: u8,
    address: String,
    phone: String,
}

impl Builder {
    #[allow(unused)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(unused)]
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    #[allow(unused)]
    pub fn set_age(&mut self, age: u8) -> &mut Self {
        self.age = age;
        self
    }

    #[allow(unused)]
    pub fn set_address(&mut self, address: &str) -> &mut Self {
        self.address = address.to_string();
        self
    }

    #[allow(unused)]
    pub fn set_phone(&mut self, phone: &str) -> &mut Self {
        self.phone = phone.to_string();
        self
    }

    #[allow(unused)]
    pub fn build(&self) -> Contact {
        Contact {
            name: self.name.clone(),
            age: self.age,
            address: self.address.clone(),
            phone: self.phone.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let contact = Builder::new()
            .set_name("Manan")
            .set_age(20)
            .set_address("Maltepe / Istanbul")
            .set_phone("555-123-2344")
            .build();

        assert_eq!(contact.name, "Manan");
        assert_eq!(contact.age, 20);
        assert_eq!(contact.address, "Maltepe / Istanbul");
        assert_eq!(contact.phone, "555-123-2344");
    }
}
