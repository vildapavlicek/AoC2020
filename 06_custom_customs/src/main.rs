mod customs;

fn main() {
    println!("Hello, world!");
    customs::Answers::load().expect("failed to load input file").count_answers();
}
