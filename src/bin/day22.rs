use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Input {
    deck1: Vec<u8>,
    deck2: Vec<u8>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut deck1 = input.deck1.clone();
    let mut deck2 = input.deck2.clone();
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.remove(0);
        let card2 = deck2.remove(0);
        if card1 > card2 {
            deck1.push(card1);
            deck1.push(card2);
        } else if card2 > card1 {
            deck2.push(card2);
            deck2.push(card1);
        }
    }
    let winner = if deck1.is_empty() { deck2 } else { deck1 };
    winner.iter().rev().enumerate().fold(0, |score,(order,card)| score + (order+1) * (*card as usize)).to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    "TBD".to_string()
}

// Day-specific code to process text data into custom problem state
#[rustfmt::skip]
fn parse_input_text(input_text: &str) -> Input {
    let mut deck1 = Vec::new();
    let mut deck2 = Vec::new();
    let mut player = 0;
    for line in input_text.lines() {
        match line {
            "Player 1:" => {
                player = 1;
            }
            "Player 2:" => {
                player = 2;
            }
            "" => {}
            _ => {
                if player == 1 {
                    deck1.push(line.parse::<u8>().unwrap());
                } else if player == 2 {
                    deck2.push(line.parse::<u8>().unwrap());
                }
            }

        }
    }
    Input{deck1,deck2}
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
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

#[test]
fn test_day22_part1() {
    process_text(_TEST_INPUT1, solve_part1, "306");
}

#[test]
fn test_day22_part2() {
    process_text(_TEST_INPUT1, solve_part2, "291");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input22.txt", solve_part1, "31314")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input22.txt", solve_part2, "???")
    );
}
