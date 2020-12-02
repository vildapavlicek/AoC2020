mod password;

use password::{Password, Passwords};

fn main() {
    let pwds = Passwords::load();
    println!(
        "1. Number of correct passwords: {}",
        pwds.count_correct_occurrences(Password::is_correct_occurrence)
    );
    //280
    println!(
        "2. Number of correct passwords: {}",
        pwds.count_correct_occurrences(Password::is_correct_position)
    );
}
