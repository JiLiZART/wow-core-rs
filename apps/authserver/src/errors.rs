use std::{error::Error, fmt::{Display, Formatter}};

#[derive(Debug)]
pub struct EnumError {
    pub name: &'static str,
    pub value: i128,
}

impl EnumError {
    pub const fn new(name: &'static str, value: i128) -> Self {
        Self { name, value }
    }
}

impl Display for EnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Enum {} can not have value: '{}'",
            self.name, self.value
        ))
    }
}

impl Error for EnumError {}
