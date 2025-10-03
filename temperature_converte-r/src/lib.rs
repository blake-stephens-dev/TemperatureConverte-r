pub mod unit;
pub fn celsius_to_fahrenheit(temp_celsius: f64) -> f64 {
    (temp_celsius * (9.0 / 5.0)) + 32.0
}

pub fn fahrenheit_to_celsius(temp_fahrenheit: f64) -> f64 {
    (temp_fahrenheit - 32.0) * (5.0 / 9.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_to_f() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(28.0), 82.4);
    }

    #[test]
    fn f_to_c() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(78.0), 25.555555555555557);
    }
}
