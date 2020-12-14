use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct Input {
    numbers: Vec<u64>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input, usize, u64) -> String;

fn is_valid_number(n: u64, history: &[u64]) -> bool {
    let len = history.len();
    for ia in 0..len - 1 {
        let a = history[ia];
        for ib in ia + 1..len {
            let b = history[ib];
            if a + b == n {
                return true;
            }
        }
    }
    false
}

// concrete instance of a ProcessInputFunc implementation
fn solve_part1(input: &Input, history_size: usize, _: u64) -> String {
    // is there a better solution than O(N^2)? who cares!
    for i in history_size..input.numbers.len() {
        let history = &input.numbers[i - history_size..i];
        let next = input.numbers[i];
        if !is_valid_number(next, history) {
            return next.to_string();
        }
    }
    String::from("All are valid!")
}

fn solve_part2(input: &Input, _: usize, target_sum: u64) -> String {
    for i0 in 0..input.numbers.len() {
        let mut sum = input.numbers[i0];
        let mut min = sum;
        let mut max = sum;
        for i1 in i0 + 1..input.numbers.len() {
            min = std::cmp::min(min, input.numbers[i1]);
            max = std::cmp::max(max, input.numbers[i1]);
            sum += input.numbers[i1];
            if sum == target_sum {
                return (min + max).to_string();
            }
            if sum > target_sum {
                break;
            }
        }
    }
    String::from("No run found!")
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    Input {
        numbers: input_text
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect(),
    }
}

fn process_text(
    input_text: &str,
    processor: ProcessInputFunc,
    history_size: usize,
    target_sum: u64,
    expected: &str,
) -> String {
    let state = parse_input_text(input_text);
    let actual = processor(&state, history_size, target_sum);
    assert_eq!(expected, actual);
    actual
}

fn process_file(
    filename: &str,
    processor: ProcessInputFunc,
    history_size: usize,
    target_sum: u64,
    expected: &str,
) -> String {
    let contents = fs::read_to_string(filename).expect(&format!("Could not load {}", filename));
    let actual = process_text(&contents, processor, history_size, target_sum, expected);
    actual
}

const _TEST_INPUT1: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

#[test]
fn test_day09_part1() {
    process_text(_TEST_INPUT1, solve_part1, 5, 0, "127");
}

#[test]
fn test_day09_part2() {
    process_text(_TEST_INPUT1, solve_part2, 5, 127, "62");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input09.txt", solve_part1, 25, 0, "144381670")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input09.txt", solve_part2, 25, 144381670, "20532569")
    );
}
