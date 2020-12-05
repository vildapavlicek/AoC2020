pub struct BoardingPasses(String);

impl BoardingPasses {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(BoardingPasses(std::fs::read_to_string("./05_binary_boarding/input.txt")?))
    }

    pub fn find_highest_id(&self) -> i64 {
        let mut ids = self.get_ids();

        ids.sort();
        ids[ids.len() - 1]
    }

    pub fn find_my_seat(&self) -> i64 {
        let mut ids = self.get_ids();
        ids.sort();

        let mut my_seat: i64 = 0;

        loop {
            if ids.contains(&(my_seat + 1)) && ids.contains(&(my_seat - 1)) && !ids.contains(&my_seat) { break; } else { my_seat += 1};
        };
            my_seat
    }

    fn get_ids(&self) -> Vec<i64> {
        self.0.lines().map(|line| {
            let s: String = line.chars().map(|c|  { if c == 'B' || c == 'R' {'1'} else {'0'}}).collect();
            i64::from_str_radix(s.as_str(), 2).unwrap()
        }).collect::<Vec<i64>>()
    }
}