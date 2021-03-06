use pgx::*;
use serde::export::Formatter;
use serde::*;
use std::fmt::Display;

#[derive(PostgresEnum, Serialize)]
pub enum SomeValue {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Display for SomeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SomeValue::One => write!(f, "1: one"),
            SomeValue::Two => write!(f, "2: two"),
            SomeValue::Three => write!(f, "3: three"),
            SomeValue::Four => write!(f, "4: four"),
            SomeValue::Five => write!(f, "5: five"),
        }
    }
}

#[pg_extern]
fn get_some_value_name(input: SomeValue) -> String {
    format!("{}", input)
}
