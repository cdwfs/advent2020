use std::collections::HashMap;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Input {
    numbers: Vec<u64>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn nth_in_sequence(input: &Input, seq_len: usize) -> String {
    let mut last_turn_spoken = HashMap::<u64, usize>::new();
    let mut prev_last_turn_spoken = HashMap::<u64, usize>::new();
    for (i, n) in input.numbers.iter().enumerate() {
        last_turn_spoken.insert(*n, i); // assumes no repetition in starting numbers
    }
    let mut n: u64 = *input.numbers.last().unwrap();
    for turn in input.numbers.len()..seq_len {
        let last = *last_turn_spoken.get(&n).unwrap();
        if !prev_last_turn_spoken.contains_key(&n) {
            // Last turn was the first time n was spoken; next n is 0.
            n = 0u64;
        } else {
            // Last turn was not the first time n was spoken
            let prev = *prev_last_turn_spoken.get(&n).unwrap();
            n = (last - prev) as u64;
        }
        if last_turn_spoken.contains_key(&n) {
            *prev_last_turn_spoken.entry(n).or_default() = *last_turn_spoken.get(&n).unwrap();
        }
        *last_turn_spoken.entry(n).or_default() = turn;
        //println!("{}: n={}, last_turn={:?}, prev_turn={:?}", turn+1, n, last_turn_spoken, prev_last_turn_spoken);
    }
    n.to_string()
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    nth_in_sequence(input, 2020)
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    nth_in_sequence(input, 30000000)
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    Input {
        numbers: input_text
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect(),
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

#[test]
fn test_day15_part1() {
    process_text("0,3,6", solve_part1, "436");
    process_text("1,3,2", solve_part1, "1");
    process_text("2,1,3", solve_part1, "10");
    process_text("1,2,3", solve_part1, "27");
    process_text("2,3,1", solve_part1, "78");
    process_text("3,2,1", solve_part1, "438");
    process_text("3,1,2", solve_part1, "1836");
}

#[test]
fn test_day15_part2() {
    // Correct, but *slow*
    //process_text("0,3,6", solve_part2, "175594");
    //process_text("1,3,2", solve_part2, "2578");
    //process_text("2,1,3", solve_part2, "3544142");
    //process_text("1,2,3", solve_part2, "261214");
    //process_text("2,3,1", solve_part2, "6895259");
    //process_text("3,2,1", solve_part2, "18");
    //process_text("3,1,2", solve_part2, "362");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input15.txt", solve_part1, "755")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input15.txt", solve_part2, "11962")
    );
}
