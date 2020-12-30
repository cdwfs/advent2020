use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs;
use std::hash::Hasher;

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
        match card1.cmp(&card2) {
            Ordering::Greater => {
                deck1.push(card1);
                deck1.push(card2);
            }
            Ordering::Less => {
                deck2.push(card2);
                deck2.push(card1);
            }
            Ordering::Equal => {
                panic!("Cards can't be equal!");
            }
        }
    }
    let winner = if deck1.is_empty() { deck2 } else { deck1 };
    winner.iter().rev().enumerate().fold(0, |score,(order,card)| score + (order+1) * (*card as usize)).to_string()
}

fn state_hash(deck1: &[u8], deck2: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    // Write lengths first, to avoid ambiguity
    hasher.write_usize(deck1.len());
    hasher.write_usize(deck2.len());
    hasher.write(deck1);
    hasher.write(deck2);
    hasher.finish()
}

fn play_game(deck1: &mut Vec<u8>, deck2: &mut Vec<u8>) -> u8 {
    //
    // To play a recursive game, each player starts with a new deck that is a COPY of the next N cards in their deck,
    // where N is the value of the card they just drew. The drawn card is not included. The deck state in the parent
    // game is preserved exactly as is until the child game (and any recursive child games) are resolved.
    let mut round_hashes: HashSet<u64> = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        // If this configuration has happened before in this GAME, player 1 wins the GAME
        // (to prevent infinite recursion).
        let hash = state_hash(deck1, deck2);
        if round_hashes.contains(&hash) {
            return 1;
        }
        round_hashes.insert(hash);
        // Otherwise, a new round of the current GAME starts. Each player draws their top card.
        let card1 = deck1.remove(0) as usize;
        let card2 = deck2.remove(0) as usize;
        // - If both players have at least as many cards left in their deck (not including the one they just drew)
        //   as the value of the card they drew, the winner of the round is determined by a recursive game.
        // - Otherwise, no recursion is possible, and the winner of the round is the player with the higher-value card.
        let winner = if deck1.len() >= card1 && deck2.len() >= card2 {
            let mut sub_deck1 = deck1[..card1].to_vec();
            let mut sub_deck2 = deck2[..card2].to_vec();
            play_game(&mut sub_deck1, &mut sub_deck2)
        } else if card1 > card2 {
            1
        } else if card2 > card1 {
            2
        } else {
            panic!("Round has no winner?!?");
        };
        // The winner of the round takes both cards and puts them on the bottom of their deck, with the winner's
        // card on top (though it may have a lower value than the losing card).
        if winner == 1 {
            deck1.push(card1 as u8);
            deck1.push(card2 as u8);
        } else if winner == 2 {
            deck2.push(card2 as u8);
            deck2.push(card1 as u8);
        }
    }
    if deck1.is_empty() {
        2
    } else {
        1
    }
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let mut deck1 = input.deck1.clone();
    let mut deck2 = input.deck2.clone();
    let winner = play_game(&mut deck1, &mut deck2);
    let winning_deck = if winner == 1 {deck1} else {deck2};
    winning_deck.iter().rev().enumerate().fold(0, |score,(order,card)| score + (order+1) * (*card as usize)).to_string()
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
        process_file("inputs/input22.txt", solve_part2, "32760")
    );
}
