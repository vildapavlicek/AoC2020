use std::convert::From;

mod data;

fn main() {
    let passports = data::Passports::from(data::AoCInput::load().expect("failed to load input"));
    let valid_passports = passports.get_valid();
    println!("Valid passports: {}", valid_passports.iter().count());
    println!(
        "Passports with valid fields: {}",
        data::with_valid_fields(valid_passports).iter().count()
    );
}
