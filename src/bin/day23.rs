use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Input {
    cups: Vec<u32>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn dec_and_wrap(x:u32, radix:u32) -> u32 {
    let x = x + radix-1; // shift to [0..radix), with an extra radix to keep the value >= 0
    let x = x-1; // decrement
    let x = x % radix; // wrap around
    x+1 // shift back to [1..9]
}

fn make_a_move(next_cups:&mut Vec<u32>, current_cup: u32) -> u32 {
    let held1 = next_cups[current_cup as usize];
    let held2 = next_cups[held1 as usize];
    let held3 = next_cups[held2 as usize];
    next_cups[current_cup as usize] = next_cups[held3 as usize];
    let mut dest_cup = dec_and_wrap(current_cup, (next_cups.len()-1) as u32);
    while dest_cup == held1 || dest_cup == held2 || dest_cup == held3 {
        dest_cup = dec_and_wrap(dest_cup, (next_cups.len()-1) as u32);
    }
    let old_next = next_cups[dest_cup as usize];
    next_cups[dest_cup as usize] = held1;
    next_cups[held3 as usize] = old_next;

    next_cups[current_cup as usize]
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut next_cups = vec![0u32;9+1];
    for i in 0..9 {
        let cup = input.cups[i];
        next_cups[cup as usize] = input.cups[(i+1) % 9];
    }
    let mut current_cup = input.cups[0];

    for _ in 1..101 {
        current_cup = make_a_move(&mut next_cups, current_cup);
    }
    let mut output = 0u64;
    let mut cup = next_cups[1];
    for _ in 0..8 {
        output = 10*output + (cup as u64);
        cup = next_cups[cup as usize];
    }
    output.to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let mut next_cups = vec![0u32;1_000_000+1];
    for (i,next_cup) in next_cups.iter_mut().enumerate() {
        *next_cup = (i+1) as u32;
    }
    for i in 0..9 {
        let cup = input.cups[i];
        next_cups[cup as usize] = input.cups[(i+1) % 9];
    }
    let mut current_cup = input.cups[0];
    next_cups[1_000_000] = current_cup;
    next_cups[input.cups[9-1] as usize] = (input.cups.len()+1) as u32;
    for _ in 1..10_000_001 {
        current_cup = make_a_move(&mut next_cups, current_cup);
    }
    let out1 = next_cups[1]as u64;
    let out2 = next_cups[out1 as usize] as u64;
    (out1*out2).to_string()
}

// Day-specific code to process text data into custom problem state
#[rustfmt::skip]
fn parse_input_text(input_text: &str) -> Input {
    Input {
        cups: input_text.bytes().map(|b| (b - b'0') as u32).collect()
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

const _TEST_INPUT1: &str = "389125467";

#[test]
fn test_day23_part1() {
    process_text(_TEST_INPUT1, solve_part1, "67384529");
}

#[test]
fn test_day23_part2() {
    process_text(_TEST_INPUT1, solve_part2, "149245887792");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input23.txt", solve_part1, "97342568")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input23.txt", solve_part2, "902208073192")
    );
}
