mod program;

fn main() {
    println!("Hello, world!");
    program::InstructionSet::parse_input().follow_instructions();
}
