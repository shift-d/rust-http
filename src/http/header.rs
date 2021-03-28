pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new(name: &str, value: &str) -> Header {
        Header {
            name: String::from(name),
            value: String::from(value),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}
