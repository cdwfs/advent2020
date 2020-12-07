use std::fs;

struct ExpenseList {
    expenses: Vec<u32>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&ExpenseList) -> String;

const TARGET_VALUE: u32 = 2020;

fn solve_part1(elist: &ExpenseList) -> String {
    for i0 in 0..elist.expenses.len() - 1 {
        let n0 = elist.expenses[i0];
        for i1 in i0 + 1..elist.expenses.len() {
            let n1 = elist.expenses[i1];
            if n0 + n1 == TARGET_VALUE {
                return (n0 * n1).to_string();
            }
        }
    }
    String::from("ERROR: no match found")
}

fn solve_part2(elist: &ExpenseList) -> String {
    for i0 in 0..elist.expenses.len() - 2 {
        let n0 = elist.expenses[i0];
        for i1 in i0 + 1..elist.expenses.len() - 1 {
            let n1 = elist.expenses[i1];
            for i2 in i0 + 1..elist.expenses.len() {
                let n2 = elist.expenses[i2];
                if n0 + n1 + n2 == TARGET_VALUE {
                    return (n0 * n1 * n2).to_string();
                }
            }
        }
    }
    String::from("ERROR: no match found")
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input: &str) -> ExpenseList {
    let mut elist = ExpenseList {
        expenses: Vec::new(),
    };
    for line in input.lines() {
        elist.expenses.push(line.parse::<u32>().unwrap());
    }
    elist
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

#[test]
fn test_part1() {
    let input = "\
1721
979
366
299
675
1456";
    process_text(input, solve_part1, "514579");
}

#[test]
fn test_part2() {
    let input = "\
1721
979
366
299
675
1456";
    process_text(input, solve_part2, "241861950");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input01.txt", solve_part1, "970816")
    );
    println!(
        "Part 1: {}",
        process_file("inputs/input01.txt", solve_part2, "96047280")
    );
}
