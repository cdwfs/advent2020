use std::collections::HashMap;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct Input {
    numbers: Vec<u32>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

// concrete instance of a ProcessInputFunc implementation
fn solve_part1(input: &Input) -> String {
    let mut ones = 0;
    let mut threes = 0;
    for i in 1..input.numbers.len() {
        match input.numbers[i] - input.numbers[i - 1] {
            1 => {
                ones += 1;
            }
            3 => {
                threes += 1;
            }
            2 => {}
            _ => panic!("unexpected sum"),
        }
    }
    (ones * threes).to_string()
}

fn solve_part2(input: &Input) -> String {
    let mut counts = HashMap::<u32, u64>::new();
    counts.insert(*input.numbers.last().unwrap(), 1u64);
    for i in (0..input.numbers.len() - 1).rev() {
        let mut count = 0u64;
        let n = input.numbers[i];
        for d in 1..4 {
            count += counts.get(&(n + d)).unwrap_or(&0u64);
        }
        counts.insert(n, count);
    }
    counts.get(&0).unwrap().to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    let mut numbers: Vec<u32> = input_text
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);
    Input { numbers }
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

const _TEST_INPUT1: &str = "\
16
10
15
5
1
11
7
19
6
12
4";

const _TEST_INPUT2: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

#[test]
fn test_day10_part1() {
    process_text(_TEST_INPUT1, solve_part1, "35");
    process_text(_TEST_INPUT2, solve_part1, "220");
}

#[test]
fn test_day10_part2() {
    process_text(_TEST_INPUT1, solve_part2, "8");
    process_text(_TEST_INPUT2, solve_part2, "19208");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input10.txt", solve_part1, "2263")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input10.txt", solve_part2, "396857386627072")
    );
}
