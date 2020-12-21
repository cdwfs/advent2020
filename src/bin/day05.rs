use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct SeatList {
    seats: Vec<String>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&SeatList) -> String;

fn get_seat_id(seat: &str) -> u32 {
    let mut row = String::new();
    let mut col = String::new();
    for b in seat[0..7].as_bytes().iter() {
        row.push(match b {
            b'F' => '0',
            b'B' => '1',
            _ => '?', // invalid
        })
    }
    for b in seat[7..].as_bytes().iter() {
        col.push(match b {
            b'L' => '0',
            b'R' => '1',
            _ => '?', // invalid
        })
    }
    let row = u32::from_str_radix(&row, 2).unwrap();
    let col = u32::from_str_radix(&col, 2).unwrap();
    row * 8 + col
}

// concrete instance of a ProcessInputFunc implementation
fn solve_part1(slist: &SeatList) -> String {
    let mut highest_seat_id = 0;
    for seat in slist.seats.iter() {
        let seat_id = get_seat_id(seat);
        highest_seat_id = std::cmp::max(highest_seat_id, seat_id);
    }
    highest_seat_id.to_string()
}

fn solve_part2(slist: &SeatList) -> String {
    let mut seat_ids = Vec::new();
    for seat in slist.seats.iter() {
        seat_ids.push(get_seat_id(seat));
    }
    seat_ids.sort();
    for i in 1..seat_ids.len() - 1 {
        if seat_ids[i - 1] + 2 == seat_ids[i] {
            return (seat_ids[i] - 1).to_string();
        }
    }
    String::from("seat not found")
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input: &str) -> SeatList {
    let mut slist = SeatList { seats: Vec::new() };
    for line in input.lines() {
        slist.seats.push(String::from(line))
    }
    slist
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

#[test]
fn test_day05_part1() {
    assert_eq!(567, get_seat_id("BFFFBBFRRR"));
    assert_eq!(119, get_seat_id("FFFBBBFRRR"));
    assert_eq!(820, get_seat_id("BBFFBBFRLL"));
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input05.txt", solve_part1, "980")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input05.txt", solve_part2, "607")
    );
}
