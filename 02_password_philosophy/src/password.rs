use serde::Deserialize;

pub struct Passwords(Vec<Password>);

impl Passwords {
    pub fn load() -> Self {
        Passwords(
            ron::from_str(
                std::fs::read_to_string("./02_password_philosophy/src/puzzle.ron")
                    .expect("failed to read puzzle data")
                    .as_str(),
            )
            .expect("failed to deserialize puzzle data"),
        )
    }

    pub fn count_correct_occurrences(&self, validate: fn(&Password) -> bool) -> i32 {
        self.0.iter().fold(
            0,
            |acc, password| if validate(password) { acc + 1 } else { acc },
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Password {
    min_occurrence: usize,
    max_occurrence: usize,
    letter: char,
    password: String,
}

impl Password {
    pub fn is_correct_occurrence(&self) -> bool {
        let occurrences_count =
            self.password
                .chars()
                .fold(0, |acc, c| if c == self.letter { acc + 1 } else { acc });

        occurrences_count >= self.min_occurrence && occurrences_count <= self.max_occurrence
    }

    pub fn is_correct_position(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();

        (chars[self.min_occurrence - 1] == self.letter
            && chars[self.max_occurrence - 1] != self.letter)
            || (chars[self.min_occurrence - 1] != self.letter
                && chars[self.max_occurrence - 1] == self.letter)
    }
}

#[test]
fn is_correct_occurrence() {
    let test_data = vec![
        (
            Password {
                max_occurrence: 3,
                min_occurrence: 1,
                letter: 'a',
                password: "abcde".to_string(),
            },
            true,
        ),
        (
            Password {
                max_occurrence: 3,
                min_occurrence: 1,
                letter: 'a',
                password: "aaabcde".to_string(),
            },
            true,
        ),
        (
            Password {
                max_occurrence: 3,
                min_occurrence: 1,
                letter: 'a',
                password: "aaaabcde".to_string(),
            },
            false,
        ),
    ];

    for (pwd, want) in test_data {
        let got = pwd.is_correct_occurrence();

        assert_eq!(want, got)
    }
}

#[test]
fn is_correct_position() {
    let test_data = vec![
        (
            Password {
                min_occurrence: 1,
                max_occurrence: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            true,
        ),
        (
            Password {
                min_occurrence: 2,
                max_occurrence: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            false,
        ),
        (
            Password {
                min_occurrence: 1,
                max_occurrence: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            false,
        ),
        (
            Password {
                min_occurrence: 1,
                max_occurrence: 3,
                letter: 'b',
                password: "bcbdefg".to_string(),
            },
            false,
        ),
    ];

    for (pwd, want) in test_data {
        let got = pwd.is_correct_position();

        assert_eq!(want, got);
    }
}
