use temperature_converte_r::unit::Unit;
fn main() {
    let temp = Unit::Fahrenheit(32.0);
    temp.display_conversion();
}
