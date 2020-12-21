use regex::Regex;
use std::collections::HashMap;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
enum Instruction<'a> {
    Mask(&'a str),
    Mem(u64, u64), // addr, value
}
#[derive(Debug)]
struct Input<'a> {
    instructions: Vec<Instruction<'a>>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut or_mask:u64 = 0;
    let mut and_mask:u64 = u64::MAX;
    let mut memory = HashMap::new();
    for inst in &input.instructions {
        match inst {
            Instruction::Mask(m) => {
                or_mask = 0;
                and_mask = u64::MAX;
                let mask_bytes = m.as_bytes();
                for i in 0..36 {
                    match mask_bytes[35-i] {
                        b'0' => { and_mask ^= 1<<i; }
                        b'1' => { or_mask |= 1<<i; }
                        b'X' => {} // no effect
                        _ => panic!("Malformed mask: {}", m)
                    }
                }
            }
            Instruction::Mem(addr,val) => {
                let masked_val = (val | or_mask) & and_mask;
                *memory.entry(addr).or_default() = masked_val;
            }
        }
    }
    memory.values().fold(0, |acc,x| acc+x).to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let mut or_mask:u64 = 0;
    let mut and_mask:u64 = u64::MAX;
    let mut x_count = 0;
    let mut x_bits = vec![0;36];
    let mut memory = HashMap::new();
    for inst in &input.instructions {
        match inst {
            Instruction::Mask(m) => {
                or_mask = 0;
                and_mask = u64::MAX;
                x_count = 0;
                let mask_bytes = m.as_bytes();
                for i in 0..36 {
                    match mask_bytes[35-i] {
                        b'0' => {} // no effect
                        b'1' => { or_mask |= 1<<i; }
                        b'X' => {
                            and_mask ^= 1<<i;
                            x_bits[x_count] = i;
                            x_count += 1;
                        }
                        _ => panic!("Malformed mask: {}", m)
                    }
                }
            }
            Instruction::Mem(addr,val) => {
                let addr = (addr | or_mask) & and_mask;
                let permutation_count = 1<<x_count;
                for i in 0..permutation_count {
                    let mut a = addr;
                    for (ib,b) in x_bits[..x_count].iter().enumerate() {
                        let bit = (i>>ib) & 1;
                        a |= bit << b;
                    }
                    *memory.entry(a).or_default() = *val;
                }
            }
        }
    }
    memory.values().fold(0, |acc,x| acc+x).to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    let mut instructions = Vec::new();
    let mem_re = Regex::new(r"^mem\[(?P<addr>\d+)\] = (?P<value>\d+)$").unwrap();
    for line in input_text.lines() {
        instructions.push(match &line[..2] {
            "ma" => {
                assert_eq!(36 + 7, line.len()); // "mask = " + 36 of X/0/1
                Instruction::Mask(&line[7..])
            }
            "me" => {
                let caps = mem_re
                    .captures(line)
                    .unwrap_or_else(|| panic!("Malformed mem instruction {}", line));
                let addr = caps.name("addr").unwrap().as_str().parse::<u64>().unwrap();
                let val = caps.name("value").unwrap().as_str().parse::<u64>().unwrap();
                Instruction::Mem(addr, val)
            }
            _ => panic!("Malformed input line {}", line),
        })
    }
    Input { instructions }
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
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

#[test]
fn test_day14_part1() {
    process_text(_TEST_INPUT1, solve_part1, "165");
}

const _TEST_INPUT2: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

#[test]
fn test_day14_part2() {
    process_text(_TEST_INPUT2, solve_part2, "208");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input14.txt", solve_part1, "11884151942312")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input14.txt", solve_part2, "2625449018811")
    );
}
