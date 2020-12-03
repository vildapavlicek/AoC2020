pub struct Map {
    map_data: Vec<Vec<char>>,
    position_x: usize,
    position_y: usize,
}

impl Map {
    pub fn move_3_1(&mut self) {
        let x = self.position_x;
        let row_length = self.map_data[self.position_y].len();

        if x + 3 >= row_length {
            self.position_x = (x + 3) - row_length;
            self.position_y += 1;
        } else {
            self.position_x = self.position_x + 3;
            self.position_y += 1;
        }
    }

    #[cfg(test)]
    pub fn get_current_position(&self) -> (usize, usize) {
        (self.position_x, self.position_y)
    }

    pub fn is_tree_at_curr_pos(&self) -> bool {
        if self.bounds_ok() {
            return self.map_data[self.position_y][self.position_x] == '#';
        }
        false
    }

    pub fn is_map_end(&self) -> bool {
        self.is_last_row() && self.is_last_column()
    }

    fn is_last_row(&self) -> bool {
        self.position_y >= self.map_data.len() - 1
    }

    fn is_last_column(&self) -> bool {
        if self.bounds_ok() {
            return self.position_x >= self.map_data[self.position_y].len() - 1;
        }
        true
    }

    fn bounds_ok(&self) -> bool {
        if self.map_data.len() - 1 >= self.position_y {
            return self.map_data[self.position_y].len() - 1 >= self.position_x;
        }
        false
    }
}

pub fn load() -> Map {
    let read_data = std::fs::read_to_string("./03_toboggan_trajectory/puzzle_input.ron")
        .expect("failed to read puzzle data");

    let map: Vec<Vec<char>> = ron::from_str(&read_data).expect("failed to deserialize");
    Map {
        map_data: map,
        position_x: 0,
        position_y: 0,
    }
}

#[test]
fn move_3_1() {
    let test_data = vec![
        (vec![vec!['.', '.'], vec!['.', '.']], (0, 0), (1, 1)),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.']],
            (0, 0),
            (3, 1),
        ),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.', '.']],
            (3, 0),
            (2, 1),
        ),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.', '.']],
            (2, 0),
            (1, 1),
        ),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.', '.']],
            (0, 0),
            (3, 1),
        ),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.', '.']],
            (3, 0),
            (2, 1),
        ),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.', '.']],
            (3, 0),
            (2, 1),
        ),
        (
            vec![
                vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
                vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
                vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
                vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#'],
            ],
            (0, 0),
            (3, 1),
        ),
        (
            vec![
                vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
                vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
                vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
                vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#'],
            ],
            (3, 1),
            (6, 2),
        ),
        (
            vec![
                vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
                vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
                vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
                vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#'],
            ],
            (6, 2),
            (9, 3),
        ),
        (
            vec![
                vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
                vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
                vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
                vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
                vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
                vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#'],
            ],
            (9, 3),
            (1, 4),
        ),
    ];

    for (map_data, (position_x, position_y), want) in test_data {
        let mut map = Map {
            position_x,
            position_y,
            map_data,
        };
        map.move_3_1();
        let got = map.get_current_position();
        assert_eq!(got, want)
    }
}

#[test]
fn is_tree_at_curr_position() {
    let test_data = vec![
        (vec![vec!['.', '.'], vec!['.', '#']], (1, 1), true),
        (vec![vec!['.', '.', '.', '#'], vec!['.', '.']], (3, 0), true),
        (vec![vec!['.', '.'], vec!['.', '.']], (1, 1), false),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.']],
            (3, 0),
            false,
        ),
    ];

    for (map_data, (position_x, position_y), want) in test_data {
        let map = Map {
            position_x,
            position_y,
            map_data,
        };
        assert_eq!(map.is_tree_at_curr_pos(), want)
    }
}

#[test]
fn is_last_column() {
    let test_data = vec![
        (vec![vec!['.', '.'], vec!['.', '#']], (1usize, 1usize), true),
        (vec![vec!['.', '.', '.', '#'], vec!['.', '.']], (2, 1), true),
        (vec![vec!['.', '.', '.', '.'], vec!['.', '.']], (3, 0), true),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.']],
            (0, 1),
            false,
        ),
    ];
    for (test_case_number, (map_data, (position_x, position_y), want)) in
        test_data.into_iter().enumerate()
    {
        let map = Map {
            position_x,
            position_y,
            map_data,
        };
        assert_eq!(map.is_last_column(), want, "test no.{}", test_case_number)
    }
}

#[test]
fn is_map_end() {
    let test_data = vec![
        (vec![vec!['.', '.'], vec!['.', '#']], (1usize, 1usize), true),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.']],
            (0, 1),
            false,
        ),
        (vec![vec!['.', '.', '.', '#'], vec!['.', '.']], (2, 1), true),
        (
            vec![vec!['.', '.', '.', '.'], vec!['.', '.']],
            (3, 0),
            false,
        ),
    ];

    for (test_case_number, (map_data, (position_x, position_y), want)) in
        test_data.into_iter().enumerate()
    {
        let map = Map {
            position_x,
            position_y,
            map_data,
        };
        assert_eq!(map.is_map_end(), want, "test no.{}", test_case_number)
    }
}

#[test]
fn tree_count() {
    let map_data = vec![
        vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
        vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
        vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
        vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
        vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
        vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
        vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
        vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
        vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#'],
    ];

    let mut map = Map {
        map_data,
        position_x: 0,
        position_y: 0,
    };
    let mut trees = 0;

    loop {
        map.move_3_1();
        if !map.is_map_end() {
            if map.is_tree_at_curr_pos() {
                trees += 1;
            }
        } else {
            break;
        }
    }

    assert_eq!(trees, 7)
}
