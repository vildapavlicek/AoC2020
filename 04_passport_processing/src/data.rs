use std::collections::HashMap;
use std::convert::From;

pub struct AoCInput(String);

impl AoCInput {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(AoCInput(std::fs::read_to_string(
            "./04_passport_processing/passports.txt",
        )?))
    }
}

#[derive(Debug)]
pub struct Passports(HashMap<usize, HashMap<String, String>>);

impl Passports {
    pub fn get_valid(&self) -> Vec<&HashMap<String, String>> {
        self.0
            .iter()
            .map(|(_, passport)| passport)
            .filter(|passport| {
                (passport.len() == 7 && passport.get("cid") == None) || passport.len() == 8
            })
            .collect::<Vec<&HashMap<String, String>>>()
    }
}

impl From<AoCInput> for Passports {
    fn from(input: AoCInput) -> Self {
        let mut m: HashMap<usize, HashMap<String, String>> = HashMap::new();
        let mut id: usize = 0;

        for line in input.0.lines() {
            if !line.is_empty() {
                line.split(' ')
                    .collect::<Vec<&str>>()
                    .iter()
                    .for_each(|&pair| {
                        let x = pair.split(':').collect::<Vec<&str>>();
                        if let Some(fields_map) = m.get_mut(&id) {
                            fields_map.insert(x[0].to_string(), x[1].to_string());
                        } else {
                            let mut new_map: HashMap<String, String> = HashMap::new();
                            new_map.insert(x[0].to_string(), x[1].to_string());
                            m.insert(id, new_map);
                        }
                    })
            } else {
                id += 1;
            }
        }

        Passports(m)
    }
}

pub fn with_valid_fields(data: Vec<&HashMap<String, String>>) -> Vec<&HashMap<String, String>> {
    data.into_iter()
        .filter(|&passport| if is_valid(passport) { true } else { false })
        .collect()
}

fn is_valid(passport: &HashMap<String, String>) -> bool {
    for (field, value) in passport {
        if !match field.as_str() {
            "byr" => is_valid_year(value, 1920, 2002),
            "iyr" => is_valid_year(value, 2010, 2020),
            "eyr" => is_valid_year(value, 2020, 2030),
            "hgt" => is_valid_hgt(value),
            "hcl" => is_valid_hcl(value),
            "ecl" => is_valid_ecl(value),
            "pid" => is_valid_pid(value),
            "cid" => true,
            _ => false,
        } {
            return false;
        }
    }
    true
}

fn is_valid_year(byr: &str, min: i32, max: i32) -> bool {
    if byr.len() != 4 {
        return false;
    }

    if let Ok(byr_i32) = byr.parse::<i32>() {
        return byr_i32 >= min && byr_i32 <= max;
    };

    false
}

fn is_valid_hcl(hcl: &str) -> bool {
    match raster::Color::hex(hcl) {
        Ok(_) => true,
        Err(err) => match err {
            raster::error::RasterError::InvalidHex => false,
            // any other error means it is valid hex but not valid color hex
            _ => true,
        },
    }
}

#[test]
fn test_is_valid_hcl() {
    assert_eq!(is_valid_hcl("#6b1c3d"), true)
}

fn is_valid_hgt(hgt: &str) -> bool {
    match hgt
        .chars()
        .skip(hgt.len() - 2)
        .take(2)
        .collect::<String>()
        .as_str()
    {
        "cm" => {
            if let Ok(value) = hgt.split_at(3).0.parse::<i32>() {
                value >= 150 && value <= 193
            } else {
                false
            }
        }
        "in" => {
            if let Ok(value) = hgt.split_at(2).0.parse::<i32>() {
                value >= 59 && value <= 76
            } else {
                false
            }
        }
        _ => false,
    }
}

#[test]
fn test_is_valid_hgt() {
    let test_cases = vec![
        ("193cm", true),
        ("110cm", false),
        ("150cm", true),
        ("195cm", false),
        ("59in", true),
        ("65in", true),
        ("45in", false),
        ("78in", false),
        ("76in", true),
    ];

    for (id, (case, want)) in test_cases.into_iter().enumerate() {
        assert_eq!(is_valid_hgt(case), want, "failed test case no. {}", id)
    }
}

fn is_valid_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn is_valid_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.parse::<i64>().is_ok()
}
