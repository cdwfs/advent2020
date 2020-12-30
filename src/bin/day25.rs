use std::fs;

#[derive(Debug)]
struct Input {
    public_keys:[u64;2],
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn get_loop_size(public_key:u64) -> u64 {
    let mut val = 1;
    let subject_number = 7;
    let mut loop_size = 0;
    while val != public_key {
        val = (val * subject_number) % 20201227;
        loop_size += 1;
    }
    loop_size
}

fn get_encryption_key(subject_number:u64, loop_size:u64) -> u64 {
    let mut val = 1;
    for _ in 0..loop_size {
        val = (val * subject_number) % 20201227;
    }
    val
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let card_public_key = input.public_keys[0];
    let door_public_key = input.public_keys[1];
    let card_loop_size = get_loop_size(card_public_key);
    let door_loop_size = get_loop_size(door_public_key);
    let card_encryption_key = get_encryption_key(door_public_key, card_loop_size);
    let door_encryption_key = get_encryption_key(card_public_key, door_loop_size);
    assert_eq!(card_encryption_key, door_encryption_key);
    card_encryption_key.to_string()
}

// Day-specific code to process text data into custom problem state
#[rustfmt::skip]
fn parse_input_text(input_text: &str) -> Input {
    let keys:Vec<&str> = input_text.lines().collect();
    Input {
        public_keys: [
            keys[0].parse::<u64>().unwrap(),
            keys[1].parse::<u64>().unwrap(),
        ],
    }
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

const _TEST_INPUT1: &str = "\
5764801
17807724";

#[test]
fn test_day25_part1() {
    process_text(_TEST_INPUT1, solve_part1, "14897079");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input25.txt", solve_part1, "16457981")
    );
}
