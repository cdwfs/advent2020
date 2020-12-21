use regex::Regex;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}
struct Input {
    instructions: Vec<Instruction>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut heading = 0;
    let mut x = 0;
    let mut y = 0;
    for inst in input.instructions.iter() {
        match inst {
            Instruction::N(arg) => { y += arg; }
            Instruction::S(arg) => { y -= arg; }
            Instruction::E(arg) => { x += arg; }
            Instruction::W(arg) => { x -= arg;}
            Instruction::L(arg) => { heading = (heading + arg) % 360; }
            Instruction::R(arg) => { heading = (heading + 360 - arg) % 360; }
            Instruction::F(arg) => {
                x = match heading {
                    0 => x+arg,
                    180 => x-arg,
                    _ => x,
                };
                y = match heading {
                    90  => y+arg,
                    270 => y-arg,
                    _ => y,
                };
            }
        }
    }
    (x.abs()+y.abs()).to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = 1;
    for inst in input.instructions.iter() {
        match inst {
            Instruction::N(arg) => { wy += arg; }
            Instruction::S(arg) => { wy -= arg; }
            Instruction::E(arg) => { wx += arg; }
            Instruction::W(arg) => { wx -= arg;}
            Instruction::L(90)  | Instruction::R(270) => { let tx = wx; wx = -wy; wy = tx; }
            Instruction::L(180) | Instruction::R(180) => { wx = -wx; wy = -wy; }
            Instruction::L(270) | Instruction::R(90)  => { let tx = -wx; wx = wy; wy = tx; }
            Instruction::F(arg) => {
                x += wx*arg;
                y += wy*arg;
            }
            _ => { panic!("Unexpected instruction {:?}", inst); }
        }
    }
    (x.abs()+y.abs()).to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    let mut instructions = Vec::new();
    let line_re = Regex::new(r"^(?P<action>[NSEWLRF])(?P<arg>\d+)$").unwrap();
    for line in input_text.lines() {
        let caps = line_re.captures(line).expect("Line didn't match regex");
        let action = caps.name("action").unwrap().as_str();
        let arg = caps.name("arg").unwrap().as_str().parse::<i32>().unwrap();
        instructions.push(match action {
            "N" => Instruction::N(arg),
            "S" => Instruction::S(arg),
            "E" => Instruction::E(arg),
            "W" => Instruction::W(arg),
            "L" => Instruction::L(arg),
            "R" => Instruction::R(arg),
            "F" => Instruction::F(arg),
            _ => Instruction::N(0),
        });
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
F10
N3
F7
R90
F11";

#[test]
fn test_day12_part1() {
    process_text(_TEST_INPUT1, solve_part1, "25");
}

#[test]
fn test_day12_part2() {
    process_text(_TEST_INPUT1, solve_part2, "286");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input12.txt", solve_part1, "879")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input12.txt", solve_part2, "18107")
    );
}
