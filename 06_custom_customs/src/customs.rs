use std::collections::{HashMap, HashSet};

pub struct Answers(String);

impl Answers {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Answers(std::fs::read_to_string(
            "./06_custom_customs/input.txt",
        )?))
    }

    pub fn count_answers(&self) {
        let answers = self.0.split("\r\n\r\n").collect::<Vec<&str>>();
        let mut answers_count = 0;
        let mut group_answers_count = 0;

        for group_answers in answers.into_iter() {
            let mut counter = std::collections::HashSet::new();
            let mut group_counter: HashMap<char, i32> = std::collections::HashMap::new();
            for person_answers in group_answers.lines() {
                person_answers.chars().for_each(|c| {
                    counter.insert(c);
                    match group_counter.get(&c) {
                        Some(count) => {
                            group_counter.insert(c, count + 1);
                        }
                        None => {
                            group_counter.insert(c, 1);
                        }
                    }
                });
            }

            answers_count += counter.len();

            let voters = group_answers.lines().count() as i32;
            group_answers_count +=
                group_counter
                    .iter()
                    .fold(0, |acc, (_, &v)| if v == voters { acc + 1 } else { acc });
        }

        println!("Answers: {}", answers_count);
        println!("Group answers: {}", group_answers_count);
    }
}
