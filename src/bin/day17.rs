use std::collections::HashSet;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Int4(i8, i8, i8, i8);

#[derive(Debug)]
struct Input {
    populated: HashSet<Int4>,
}

#[rustfmt::skip]
static CELL_DELTAS_3D: &[Int4] = &[
    Int4(-1,-1,-1, 0),
    Int4(-1,-1, 0, 0),
    Int4(-1,-1, 1, 0),
    Int4(-1, 0,-1, 0),
    Int4(-1, 0, 0, 0),
    Int4(-1, 0, 1, 0),
    Int4(-1, 1,-1, 0),
    Int4(-1, 1, 0, 0),
    Int4(-1, 1, 1, 0),
    Int4( 0,-1,-1, 0),
    Int4( 0,-1, 0, 0),
    Int4( 0,-1, 1, 0),
    Int4( 0, 0,-1, 0),
    //Int4( 0, 0, 0, 0), // skip identity delta
    Int4( 0, 0, 1, 0),
    Int4( 0, 1,-1, 0),
    Int4( 0, 1, 0, 0),
    Int4( 0, 1, 1, 0),
    Int4( 1,-1,-1, 0),
    Int4( 1,-1, 0, 0),
    Int4( 1,-1, 1, 0),
    Int4( 1, 0,-1, 0),
    Int4( 1, 0, 0, 0),
    Int4( 1, 0, 1, 0),
    Int4( 1, 1,-1, 0),
    Int4( 1, 1, 0, 0),
    Int4( 1, 1, 1, 0),
];

#[rustfmt::skip]
static CELL_DELTAS_4D: &[Int4] = &[
    Int4(-1,-1,-1,-1),
    Int4(-1,-1, 0,-1),
    Int4(-1,-1, 1,-1),
    Int4(-1, 0,-1,-1),
    Int4(-1, 0, 0,-1),
    Int4(-1, 0, 1,-1),
    Int4(-1, 1,-1,-1),
    Int4(-1, 1, 0,-1),
    Int4(-1, 1, 1,-1),
    Int4( 0,-1,-1,-1),
    Int4( 0,-1, 0,-1),
    Int4( 0,-1, 1,-1),
    Int4( 0, 0,-1,-1),
    Int4( 0, 0, 0,-1),
    Int4( 0, 0, 1,-1),
    Int4( 0, 1,-1,-1),
    Int4( 0, 1, 0,-1),
    Int4( 0, 1, 1,-1),
    Int4( 1,-1,-1,-1),
    Int4( 1,-1, 0,-1),
    Int4( 1,-1, 1,-1),
    Int4( 1, 0,-1,-1),
    Int4( 1, 0, 0,-1),
    Int4( 1, 0, 1,-1),
    Int4( 1, 1,-1,-1),
    Int4( 1, 1, 0,-1),
    Int4( 1, 1, 1,-1),

    Int4(-1,-1,-1, 0),
    Int4(-1,-1, 0, 0),
    Int4(-1,-1, 1, 0),
    Int4(-1, 0,-1, 0),
    Int4(-1, 0, 0, 0),
    Int4(-1, 0, 1, 0),
    Int4(-1, 1,-1, 0),
    Int4(-1, 1, 0, 0),
    Int4(-1, 1, 1, 0),
    Int4( 0,-1,-1, 0),
    Int4( 0,-1, 0, 0),
    Int4( 0,-1, 1, 0),
    Int4( 0, 0,-1, 0),
    //Int4( 0, 0, 0, 0), // skip identity delta
    Int4( 0, 0, 1, 0),
    Int4( 0, 1,-1, 0),
    Int4( 0, 1, 0, 0),
    Int4( 0, 1, 1, 0),
    Int4( 1,-1,-1, 0),
    Int4( 1,-1, 0, 0),
    Int4( 1,-1, 1, 0),
    Int4( 1, 0,-1, 0),
    Int4( 1, 0, 0, 0),
    Int4( 1, 0, 1, 0),
    Int4( 1, 1,-1, 0),
    Int4( 1, 1, 0, 0),
    Int4( 1, 1, 1, 0),

    Int4(-1,-1,-1, 1),
    Int4(-1,-1, 0, 1),
    Int4(-1,-1, 1, 1),
    Int4(-1, 0,-1, 1),
    Int4(-1, 0, 0, 1),
    Int4(-1, 0, 1, 1),
    Int4(-1, 1,-1, 1),
    Int4(-1, 1, 0, 1),
    Int4(-1, 1, 1, 1),
    Int4( 0,-1,-1, 1),
    Int4( 0,-1, 0, 1),
    Int4( 0,-1, 1, 1),
    Int4( 0, 0,-1, 1),
    Int4( 0, 0, 0, 1),
    Int4( 0, 0, 1, 1),
    Int4( 0, 1,-1, 1),
    Int4( 0, 1, 0, 1),
    Int4( 0, 1, 1, 1),
    Int4( 1,-1,-1, 1),
    Int4( 1,-1, 0, 1),
    Int4( 1,-1, 1, 1),
    Int4( 1, 0,-1, 1),
    Int4( 1, 0, 0, 1),
    Int4( 1, 0, 1, 1),
    Int4( 1, 1,-1, 1),
    Int4( 1, 1, 0, 1),
    Int4( 1, 1, 1, 1),
];

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn active_cells_after_n_steps(input: &Input, deltas: &[Int4], gen_count: u32) -> usize {
    assert!(
        gen_count < 100,
        "coordinates are only 8 bit and are likely to overflow"
    );

    let mut state = input.populated.clone();
    let mut to_check: HashSet<Int4> = HashSet::with_capacity(state.len() * deltas.len());
    // Populate to_check from initial state. In the main loop, it's generated
    // alongside the new state.
    for cell in &state {
        to_check.insert(*cell);
        for delta in deltas {
            to_check.insert(Int4(
                cell.0 + delta.0,
                cell.1 + delta.1,
                cell.2 + delta.2,
                cell.3 + delta.3,
            ));
        }
    }
    for _ in 0..gen_count {
        let mut next_state: HashSet<Int4> = HashSet::with_capacity(to_check.len());
        let mut next_to_check: HashSet<Int4> =
            HashSet::with_capacity(next_state.len() * deltas.len());
        for cell in &to_check {
            // count live neighbors (stopping at 4, as that doesn't make a difference)
            let mut live_neighbor_count = 0;
            for delta in deltas {
                if state.contains(&Int4(
                    cell.0 + delta.0,
                    cell.1 + delta.1,
                    cell.2 + delta.2,
                    cell.3 + delta.3,
                )) {
                    live_neighbor_count += 1;
                    if live_neighbor_count > 3 {
                        break;
                    }
                }
            }
            if live_neighbor_count == 3 || (live_neighbor_count == 2 && state.contains(cell)) {
                // cell stays/becomes alive; check it and its neighbors next time
                next_state.insert(*cell);
                next_to_check.insert(*cell);
                for delta in deltas {
                    next_to_check.insert(Int4(
                        cell.0 + delta.0,
                        cell.1 + delta.1,
                        cell.2 + delta.2,
                        cell.3 + delta.3,
                    ));
                }
            }
        }
        state = next_state;
        to_check = next_to_check;
    }
    state.len()
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    active_cells_after_n_steps(input, CELL_DELTAS_3D, 6).to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    active_cells_after_n_steps(input, CELL_DELTAS_4D, 6).to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    let mut populated = HashSet::new();
    for (y, line) in input_text.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            if *c == b'#' {
                populated.insert(Int4(x as i8, y as i8, 0i8, 0i8));
            }
        }
    }
    Input { populated }
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
.#.
..#
###";

#[test]
fn test_day17_part1() {
    process_text(_TEST_INPUT1, solve_part1, "112");
}

#[test]
fn test_day17_part2() {
    process_text(_TEST_INPUT1, solve_part2, "848");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input17.txt", solve_part1, "242")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input17.txt", solve_part2, "2292")
    );
}
