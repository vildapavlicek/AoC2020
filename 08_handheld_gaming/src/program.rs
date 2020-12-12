use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};

pub struct InstructionSet(Vec<Instruction>);

impl InstructionSet {
    pub fn parse_input() -> Self {
        let input = std::fs::read_to_string("./08_handheld_gaming/input.txt").expect("failed to load input file");

        let mut instructions = Vec::with_capacity(621);

        for line in input.lines() {
            let instruction = line.split(' ').collect::<Vec<&str>>();
           instructions.push(Instruction {kind: instruction[0].to_owned(), value: instruction[1].parse::<i64>().expect("failed to parse value"), visited: RefCell::new(false)});
           }

        InstructionSet(instructions)
    }

    pub fn follow_instructions(&mut self) {
        let mut instruction = self.0[0].borrow();
        let mut i: i64 = 0;
        let mut acc = 0;

        while !instruction.is_visited() {
            instruction.set_visited();
            match instruction.kind.as_str() {
                "acc" => { acc += instruction.value; i += 1; instruction = self.0[i  as usize].borrow_mut(); },
                "jmp" => {i += instruction.value; instruction = self.0[i  as usize].borrow_mut()},
                "nop" => {i += 1; instruction = self.0[i  as usize].borrow_mut()},
                _ => (),
            }
        }

        println!("{}", acc);

    }
}

#[derive(Debug)]
pub struct Instruction {
    kind: String,
    value: i64,
    visited: RefCell<bool>,
}

impl Instruction {
    pub fn set_visited(&self) {
        self.visited.replace(true);
    }

    pub fn is_visited(&self) -> bool {
        self.visited.borrow().eq(&true)
    }
}
