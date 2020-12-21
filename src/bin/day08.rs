use regex::Regex;
use std::collections::HashSet;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct Program {
    instructions: Vec<Instruction>,
}
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Program) -> String;

// concrete instance of a ProcessInputFunc implementation
fn solve_part1(input: &Program) -> String {
    let mut acc = 0;
    let mut executed = HashSet::new();
    let mut ip: isize = 0;
    loop {
        if executed.contains(&ip) {
            break;
        }
        executed.insert(ip);
        match input.instructions[ip as usize] {
            Instruction::NOP(_) => ip += 1,
            Instruction::ACC(arg) => {
                acc += arg;
                ip += 1
            }
            Instruction::JMP(arg) => ip += arg,
        }
    }
    acc.to_string()
}

fn solve_part2(input: &Program) -> String {
    for ip_switch in 0..input.instructions.len() {
        let mut acc = 0;
        let mut executed = HashSet::new();
        let mut ip: isize = 0;
        loop {
            if ip as usize >= input.instructions.len() {
                return acc.to_string();
            }
            if executed.contains(&ip) {
                break; // infinite loop
            }
            executed.insert(ip);
            let mut inst = &input.instructions[ip as usize];
            let mut _inst_swap = Instruction::NOP(0);
            if ip as usize == ip_switch {
                _inst_swap = match *inst {
                    Instruction::NOP(arg) => Instruction::JMP(arg),
                    Instruction::JMP(arg) => Instruction::NOP(arg),
                    Instruction::ACC(arg) => Instruction::ACC(arg),
                };
                inst = &_inst_swap;
            }
            match inst {
                Instruction::NOP(_) => ip += 1,
                Instruction::ACC(arg) => {
                    acc += arg;
                    ip += 1
                }
                Instruction::JMP(arg) => ip += arg,
            }
        }
    }
    String::from("No program terminated")
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input: &str) -> Program {
    let mut instructions = Vec::new();
    let inst_re = Regex::new(r"^(?P<op>[a-z]{3}) (?P<arg>[\-+]\d+)$").unwrap();
    for line in input.lines() {
        let inst_caps = inst_re.captures(line).unwrap();
        let op = inst_caps.name("op").unwrap().as_str();
        let arg = inst_caps
            .name("arg")
            .unwrap()
            .as_str()
            .parse::<isize>()
            .unwrap();
        instructions.push(match op {
            "nop" => Instruction::NOP(arg),
            "acc" => Instruction::ACC(arg),
            "jmp" => Instruction::JMP(arg),
            _ => {
                panic!("Invalid opcode {}", op);
            }
        })
    }
    Program { instructions }
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
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

#[test]
fn test_day08_part1() {
    process_text(_TEST_INPUT1, solve_part1, "5");
}

#[test]
fn test_day08_part2() {
    process_text(_TEST_INPUT1, solve_part2, "8");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input08.txt", solve_part1, "1217")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input08.txt", solve_part2, "501")
    );
}
