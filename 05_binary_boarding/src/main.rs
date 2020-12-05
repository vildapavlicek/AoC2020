mod boarding_pass;

fn main() {
    println!("Hello, world!");
    let passes = boarding_pass::BoardingPasses::load().expect("failed to load input file");
    println!("{}", passes.find_highest_id());
    println!("{}", passes.find_my_seat());
}
