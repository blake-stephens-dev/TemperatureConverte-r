use crate::{celsius_to_fahrenheit, fahrenheit_to_celsius};
use std::fmt;
#[derive(Debug)]
pub enum Unit {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Unit {
    pub fn convert(&self) -> Unit {
        match self {
            Unit::Fahrenheit(temp) => Unit::Celsius(fahrenheit_to_celsius(*temp)),
            Unit::Celsius(temp) => Unit::Fahrenheit(celsius_to_fahrenheit(*temp)),
        }
    }

    pub fn display_conversion(&self) {
        let convert = self.convert();
        println!("Self: {}, Converted: {}", self, convert);
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Celsius(c) => write!(f, "{:.2} °C", c),
            Unit::Fahrenheit(ft) => write!(f, "{:.2} °F", ft),
        }
    }
}
