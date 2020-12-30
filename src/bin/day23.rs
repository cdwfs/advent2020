use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Input {
    cups: Vec<u8>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn inc_index(x:usize) -> usize {
    (x+1) % 9
}
fn dec_cup(x:u8) -> u8 {
    let x = x + 9-1; // shift to [0..9), with an extra 9 to keep the value >= 0
    let x = x-1; // decrement
    let x = x % 9; // wrap around
    x+1 // shift back to [1..9]
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut cups:Vec<u8> = input.cups.clone();
    let mut held_cups:Vec<u8> = Vec::with_capacity(3);
    let mut unheld_cups:Vec<u8> = Vec::with_capacity(6);
    let mut take_indices:Vec<usize> = Vec::with_capacity(3);
    let mut current_index = 0;
    for i_move in 1..101 {
        let current_cup = cups[current_index];

        // The crab picks up the three cups that are immediately clockwise
        // of the current cup. They are removed from the circle; cup spacing
        // is adjusted as necessary to maintain the circle.
        assert_eq!(held_cups.capacity() + unheld_cups.capacity(), cups.len());
        held_cups.clear();
        unheld_cups.clear();
        take_indices.clear();
        let mut take_index = current_index;
        for _ in 0..3 {
            take_index = inc_index(take_index);
            take_indices.push(take_index);
            held_cups.push(cups[take_index]);
        }
        for (i,cup) in cups.iter().enumerate() {
            if take_indices[0] != i && take_indices[1] != i && take_indices[2] != i {
                unheld_cups.push(*cup);
            }
        }
        assert_eq!(3, held_cups.len());
        assert_eq!(6, unheld_cups.len());

        // The crab selects a destination cup: the cup with a label equal to the
        // current cup's label minus one. If this would select one of the cups
        // that was just picked up, the crab will keep subtracting one until it
        // finds a cup that wasn't just picked up. If at any point in this process
        // the value goes below the lowest value on any cup's label, it wraps around
        // to the highest value on any cup's label instead.
        let mut dest_cup = dec_cup(current_cup);
        while held_cups[0] == dest_cup || held_cups[1] == dest_cup || held_cups[2] == dest_cup {
            dest_cup = dec_cup(dest_cup);
        }
        /*
        println!("-- move {} --", i_move);
        print!("cups: ");
        for cup in cups.iter() {
            if *cup == current_cup {
                print!("({}) ", *cup);
            } else {
                print!("{} ", *cup);
            }
        }
        println!("\npick up: {:?}", held_cups);
        println!("destination: {}\n", dest_cup);
        */
        // The crab places the cups it just picked up so that they are immediately
        // clockwise of the destination cup. They keep the same order as when they
        // were picked up.
        cups.clear();
        current_index = 10; // invalid
        for cup in unheld_cups.iter() {
            if *cup == current_cup {
                // Track the index of the current cup in the new array
                assert_eq!(current_index, 10);
                current_index = cups.len();
            }
            cups.push(*cup);
            if *cup == dest_cup {
                cups.push(held_cups[0]);
                cups.push(held_cups[1]);
                cups.push(held_cups[2]);
            }
        }
        // The crab selects a new current cup: the cup which is immediately clockwise
        // of the current cup.
        current_index = inc_index(current_index);
    }
    let mut output = 0u64;
    let mut index = inc_index(cups.iter().position(|cup| *cup == 1).unwrap());
    for _ in 0..8 {
        output = 10*output + (cups[index] as u64);
        index = inc_index(index);
    }
    output.to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    "TBD".to_string()
}

// Day-specific code to process text data into custom problem state
#[rustfmt::skip]
fn parse_input_text(input_text: &str) -> Input {
    Input {
        cups: input_text.bytes().map(|b| b - b'0').collect()
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
        process_file("inputs/input23.txt", solve_part1, "???")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input23.txt", solve_part2, "???")
    );
}
