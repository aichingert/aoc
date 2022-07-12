fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: i32,
}

impl Passport {
    fn clear() -> Self {
        Self { 
            byr: -1,
            iyr: -1, 
            eyr: -1, 
            hgt: "".to_string(), 
            hcl: "".to_string(), 
            ecl: "".to_string(), 
            pid: "".to_string(),
            cid: -1,
        }
    }
}

fn solve_part_one(input: &String) {
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport: Passport = Passport::clear();
    let mut is_valid: bool;
    let mut counter: i32 = 0;

    for line in input.lines() {
        if line == "" {
            passports.push(passport.clone());
            passport = Passport::clear();
            continue;
        }

        let values: Vec<&str> = line.split(' ').collect();

        for split in values {
            let pair: Vec<&str> = split.split(':').collect();

            match pair[0] {
                "byr" => {
                    passport.byr = pair[1].parse().unwrap();
                },
                "iyr" => {
                    passport.iyr = pair[1].parse().unwrap();
                },
                "eyr" => {
                    passport.eyr = pair[1].parse().unwrap();
                },
                "hgt" => {
                    passport.hgt = pair[1].to_string();
                },
                "hcl" => {
                    passport.hcl = pair[1].to_string();
                },
                "ecl" => {
                    passport.ecl = pair[1].to_string();
                },
                "pid" => {
                    passport.pid = pair[1].to_string();
                },
                "cid" => {
                    passport.cid = pair[1].parse().unwrap();
                }
                _ => {}
            }
        }
    }

    passports.push(passport.clone());

    let mut offset: usize = 0;
    
    for i in 0..passports.len() {
        if passports[i - offset].byr == -1 || passports[i - offset].iyr == -1 || passports[i - offset].eyr == -1 || passports[i - offset].hgt == "".to_string() || passports[i - offset].hcl == "".to_string() || passports[i - offset].ecl == "".to_string() || passports[i - offset].pid == "".to_string() {
            is_valid = false;
        } else {
            is_valid = true;
        }

        if is_valid {
            counter += 1;
        } else {
            passports.remove(i - offset);
            offset += 1;
        }
    }

    println!("Solution part one: {}", counter);

    solve_part_two(&mut passports);
}


fn solve_part_two(passports: &mut Vec<Passport>) {
    let mut is_valid: bool;
    let mut counter: i32 = 0;

    for i in 0..passports.len() {
        if passports[i].byr.to_string().len() == 4 && passports[i].byr >= 1920 && passports[i].byr <= 2002
        && passports[i].iyr.to_string().len() == 4 && passports[i].iyr >= 2010 && passports[i].iyr <= 2020
        && passports[i].eyr.to_string().len() == 4 && passports[i].eyr >= 2020 && passports[i].eyr <= 2030
        && check_height(&passports[i]) && check_hair_color(&passports[i])
        && check_eye_color(&passports[i]) && check_pid(&passports[i]) {
            is_valid = true;
        } else {
            is_valid = false;
        }

        check_hair_color(&passports[i]);

        if is_valid {
            counter += 1;
        }
    }


    println!("Solution part two: {}", counter);
}

fn check_height(passport: &Passport) -> bool {
    let mut n: String = String::new();
    let mut format: String = String::new();

    for c in passport.hgt.chars() {
        if c.is_numeric() {
            n.push(c)
        } else {
            format.push(c);
        }
    }

    let number: i32 = n.parse().unwrap();

    match &format as &str {
        "in" => {
            if number >= 59 && number <= 76 {
                return true;
            }
        },
        "cm" => {
            if number >= 150 && number <= 193 {
                return true;
            }
        },
        _ => {}
    }

    false
}


fn check_hair_color(passport: &Passport) -> bool {
    let mut hair_color: String = passport.hcl.clone();
    let mut is_valid: bool = false;

    if hair_color.remove(0) != '#' {
        return false;
    }

    if hair_color.len() != 6 {
        return false;
    }

    for c in hair_color.chars() {
        for check in 'a'..='f' {
            if c == check {
                is_valid = true;
            }
        }

        if c.is_numeric() {
            is_valid = true;
        }
    }

    is_valid
}

fn check_eye_color(passport: &Passport) -> bool {
    match &passport.ecl as &str {
        "amb" => {
            return true;
        },
        "blu" => {
            return true;
        },
        "brn" => {
            return true;
        },
        "gry" => {
            return true;
        },
        "grn" => {
            return true;
        }
        "hzl" => {
            return true;
        },
        "oth" => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

fn check_pid(passport: &Passport) -> bool {
    if passport.pid.len() != 9 {
        return false;
    }

    for c in passport.pid.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    true
}