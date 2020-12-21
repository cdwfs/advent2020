use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Passport {
    fields: HashMap<String, String>,
}
impl Passport {
    fn new() -> Passport {
        Passport {
            fields: HashMap::new(),
        }
    }
}
// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct PassportList {
    passports: Vec<Passport>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&PassportList) -> String;

// concrete instance of a ProcessInputFunc implementation
fn solve_part1(plist: &PassportList) -> String {
    let required_fields = vec![
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
    ];
    let mut valid_count = 0;
    for passport in plist.passports.iter() {
        let mut valid = true;
        for req in required_fields.iter() {
            if !passport.fields.contains_key(req) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_count += 1;
        }
    }
    valid_count.to_string()
}

fn solve_part2(plist: &PassportList) -> String {
    let required_fields = vec![
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
    ];
    let hgt_re = Regex::new(r"^(?P<height>\d{2,3})(?P<unit>cm|in)$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let mut valid_count = 0;
    for passport in plist.passports.iter() {
        let mut valid = true;
        // Check for exisence of required fields
        for req in required_fields.iter() {
            if !passport.fields.contains_key(req) {
                valid = false;
                break;
            }
        }
        if !valid {
            continue;
        }
        // Field-specific validation.
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        let byr = passport.fields.get("byr").unwrap();
        let byr = byr.parse::<u32>().unwrap();
        if byr < 1920 || byr > 2002 {
            continue;
        }
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        let iyr = passport.fields.get("iyr").unwrap();
        let iyr = iyr.parse::<u32>().unwrap();
        if iyr < 2010 || iyr > 2020 {
            continue;
        }
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let eyr = passport.fields.get("eyr").unwrap();
        let eyr = eyr.parse::<u32>().unwrap();
        if eyr < 2020 || eyr > 2030 {
            continue;
        }
        // hgt (Height) - a number followed by either cm or in:
        //    If cm, the number must be at least 150 and at most 193.
        //    If in, the number must be at least 59 and at most 76.
        let hgt = passport.fields.get("hgt").unwrap();
        match hgt_re.captures(hgt) {
            Some(caps) => {
                let height = caps.name("height").unwrap().as_str();
                let height = height.parse::<u32>().unwrap();
                let unit = caps.name("unit").unwrap().as_str();
                if unit == "cm" && (height < 150 || height > 193) {
                    continue;
                }
                if unit == "in" && (height < 59 || height > 76) {
                    continue;
                }
            }
            None => {
                continue;
            }
        }
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let hcl = passport.fields.get("hcl").unwrap();
        if !hcl_re.is_match(hcl) {
            continue;
        }
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let ecl = passport.fields.get("ecl").unwrap();
        if !ecl_re.is_match(ecl) {
            continue;
        }
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let pid = passport.fields.get("pid").unwrap();
        if !pid_re.is_match(pid) {
            continue;
        }
        // cid (Country ID) - ignored, missing or not.

        valid_count += 1;
    }
    valid_count.to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input: &str) -> PassportList {
    let mut plist = PassportList {
        passports: Vec::new(),
    };
    let re = Regex::new(r"^(?P<field>\w{3}):(?P<value>\S+)$").unwrap();
    let mut pip = Passport::new();
    for line in input.lines() {
        if line.is_empty() {
            // Add the current passport to the list and start a new one
            plist.passports.push(pip);
            pip = Passport::new();
        }
        let field_values = line.split_ascii_whitespace();
        for pair in field_values {
            assert!(re.is_match(pair), "input did not match regex: {}", pair);
            let caps = re.captures(pair).unwrap();
            let field = caps.name("field").unwrap().as_str();
            let value = caps.name("value").unwrap().as_str();
            match field {
                "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" | "cid" => {
                    pip.fields.insert(String::from(field), String::from(value))
                }
                _ => panic!("Unrecognized passport field: {}", field),
            };
        }
    }
    // Add the last passport, if one was in progress
    if !pip.fields.is_empty() {
        plist.passports.push(pip);
    }
    plist
}

fn process_text(input_text: &str, processor: ProcessInputFunc, expected: &str) -> String {
    let state = parse_input_text(input_text);
    let actual = processor(&state);
    assert_eq!(expected, actual);
    actual
}

fn process_file(filename: &str, processor: ProcessInputFunc, expected: &str) -> String {
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Could not load {}", filename));
    process_text(&contents, processor, expected)
}

#[test]
fn test_day04_part1() {
    const TEST_INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    process_text(TEST_INPUT, solve_part1, "2");
}

#[test]
fn test_day04_part2_invalid() {
    const TEST_INVALID_PASSPORTS: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
    process_text(TEST_INVALID_PASSPORTS, solve_part2, "0");
}
#[test]
fn test_day04_part2_valid() {
    const TEST_VALID_PASSPORTS: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    process_text(TEST_VALID_PASSPORTS, solve_part2, "4");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input04.txt", solve_part1, "237")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input04.txt", solve_part2, "172")
    );
}
