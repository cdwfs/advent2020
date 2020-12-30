use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Input {
    cups: Vec<usize>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn dec_and_wrap(x:usize, radix:usize) -> usize {
    let x = x + radix-1; // shift to [0..radix), with an extra radix to keep the value >= 0
    let x = x-1; // decrement
    let x = x % radix; // wrap around
    x+1 // shift back to [1..9]
}

fn make_a_move(next_cups:&mut Vec<usize>, current_cup: usize) -> usize {
    let held1 = next_cups[current_cup];
    let held2 = next_cups[held1];
    let held3 = next_cups[held2];
    next_cups[current_cup] = next_cups[held3];
    let mut dest_cup = dec_and_wrap(current_cup, next_cups.len()-1);
    while dest_cup == held1 || dest_cup == held2 || dest_cup == held3 {
        dest_cup = dec_and_wrap(dest_cup, next_cups.len()-1);
    }
    let old_next = next_cups[dest_cup];
    next_cups[dest_cup] = held1;
    next_cups[held3] = old_next;

    next_cups[current_cup]
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut next_cups = vec![0usize;9+1];
    for i in 0..9 {
        let cup = input.cups[i];
        next_cups[cup] = input.cups[(i+1) % 9];
    }
    let mut current_cup = input.cups[0];

    for _ in 1..101 {
        current_cup = make_a_move(&mut next_cups, current_cup);
    }
    let mut output = 0u64;
    let mut cup = next_cups[1];
    for _ in 0..8 {
        output = 10*output + (cup as u64);
        cup = next_cups[cup];
    }
    output.to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let mut next_cups = vec![0usize;1_000_000+1];
    for (i,next_cup) in next_cups.iter_mut().enumerate() {
        *next_cup = i+1;
    }
    for i in 0..9 {
        let cup = input.cups[i];
        next_cups[cup] = input.cups[(i+1) % 9];
    }
    let mut current_cup = input.cups[0];
    next_cups[1_000_000] = current_cup;
    next_cups[input.cups[9-1]] = input.cups.len()+1;
    for _ in 1..10_000_001 {
        current_cup = make_a_move(&mut next_cups, current_cup);
    }
    let out1 = next_cups[1];
    let out2 = next_cups[out1];
    (out1*out2).to_string()
}

// Day-specific code to process text data into custom problem state
#[rustfmt::skip]
fn parse_input_text(input_text: &str) -> Input {
    Input {
        cups: input_text.bytes().map(|b| (b - b'0') as usize).collect()
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
