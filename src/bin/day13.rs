use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Input {
    t0:u64,
    bus_ids:Vec<Option<u64>>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut td = std::u64::MAX;
    let mut first_bus_id = 0u64;
    for bus in input.bus_ids.iter() {
        match bus {
            None => {continue;}
            Some(bus_id) => {
                let x = input.t0 / bus_id;
                let mut t = bus_id * x;
                if t < input.t0 {
                    t += bus_id;
                }
                if t < td {
                    td = t;
                    first_bus_id = *bus_id;
                }
            }
        }
    }
    ((td-input.t0) * first_bus_id).to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let id_count = input.bus_ids.len() as u64;
    let mut stop = input.bus_ids[0].unwrap();
    let mut prod = stop;
    for i in 1..input.bus_ids.len() {
        let mut n = stop + 1;
        stop = match input.bus_ids[i] {
            None => n,
            Some(id) => {
                while (n % id) != 0 {
                    n += prod;
                }
                prod *= id;
                n
            }
        }
    }
    (stop - id_count + 1).to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    let lines:Vec<&str> = input_text.lines().collect();
    let t0 = lines[0].parse::<u64>().unwrap();
    let mut bus_ids = Vec::new();
    for bus in lines[1].split(",") {
        bus_ids.push(match bus {
            "x" => None,
            id => Some(id.parse::<u64>().unwrap()),
        });
    }
    Input { t0, bus_ids }
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
939
7,13,x,x,59,x,31,19";

#[test]
fn test_day13_part1() {
    process_text(_TEST_INPUT1, solve_part1, "295");
}

#[test]
fn test_day13_part2() {
    process_text(_TEST_INPUT1, solve_part2, "1068781");
    process_text("0\n17,x,13,19", solve_part2, "3417");
    process_text("0\n67,7,59,61", solve_part2, "754018");
    process_text("0\n67,x,7,59,61", solve_part2, "779210");
    process_text("0\n67,7,x,59,61", solve_part2, "1261476");
    process_text("0\n1789,37,47,1889", solve_part2, "1202161486");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input13.txt", solve_part1, "3269")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input13.txt", solve_part2, "672754131923874")
    );
}
