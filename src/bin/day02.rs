use regex::Regex;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct PasswordList {
    passwords: Vec<Password>,
}
struct Password {
    n1: usize,
    n2: usize,
    c: u8, // NOTE: u8, not char. We'll hold our nose and assume ASCII text for AoC, for simplicity.
    password: String,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&PasswordList) -> String;

// concrete instance of a ProcessInputFunc implementation
fn count_valid_passwords_part1(plist: &PasswordList) -> String {
    let mut valid_count = 0;
    for pw in &plist.passwords {
        let mut c_count = 0;
        for c in pw.password.as_bytes() {
            if c == &pw.c {
                c_count += 1;
            }
        }
        if c_count >= pw.n1 && c_count <= pw.n2 {
            valid_count += 1;
        }
    }
    valid_count.to_string()
}

fn count_valid_passwords_part2(plist: &PasswordList) -> String {
    let mut valid_count = 0;
    for pw in &plist.passwords {
        // Previous char-based indexing solution, for reference:
        //let n1_is_c = pw.password.chars().nth(pw.n1-1).unwrap() == pw.c;
        let n1_is_c = pw.password.as_bytes()[pw.n1 - 1] == pw.c;
        let n2_is_c = pw.password.as_bytes()[pw.n2 - 1] == pw.c;
        if (n1_is_c && !n2_is_c) || (!n1_is_c && n2_is_c) {
            valid_count += 1;
        }
    }
    valid_count.to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input: &str) -> PasswordList {
    let mut plist = PasswordList {
        passwords: Vec::new(),
    };
    let re = Regex::new(r"^(?P<n1>\d+)-(?P<n2>\d+) (?P<c>[a-z]): (?P<pw>[a-z]+)$").unwrap();
    for line in input.lines() {
        assert!(re.is_match(line));
        let caps = re.captures(line).unwrap();
        plist.passwords.push(Password {
            n1: caps.name("n1").unwrap().as_str().parse::<usize>().unwrap(),
            n2: caps.name("n2").unwrap().as_str().parse::<usize>().unwrap(),
            c: caps.name("c").unwrap().as_str().as_bytes()[0],
            password: String::from(caps.name("pw").unwrap().as_str()),
        });
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
    let contents = fs::read_to_string(filename).expect(&format!("Could not load {}", filename));
    let actual = process_text(&contents, processor, expected);
    actual
}

#[test]
fn test_day02_part1() {
    let input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    process_text(input, count_valid_passwords_part1, "2");
}

#[test]
fn test_day02_part2() {
    let input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    process_text(input, count_valid_passwords_part2, "1");
}
fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input02.txt", count_valid_passwords_part1, "591")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input02.txt", count_valid_passwords_part2, "335")
    );
}
