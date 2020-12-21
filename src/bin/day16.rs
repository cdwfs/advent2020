use regex::Regex;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Field<'a> {
    name: &'a str,
    min1: u32,
    max1: u32,
    min2: u32,
    max2: u32,
}
#[derive(Debug)]
struct Input<'a> {
    fields: Vec<Field<'a>>,
    your_ticket: Vec<u32>,
    other_tickets: Vec<Vec<u32>>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut error_rate = 0;
    for ticket in input.other_tickets.iter() {
        for val in ticket.iter() {
            let mut found_valid_field = false;
            for field in input.fields.iter() {
                if (*val >= field.min1 && *val <= field.max1) || (*val >= field.min2 && *val <= field.max2) {
                    found_valid_field = true;
                    break;
                }
            }
            if !found_valid_field {
                error_rate += *val;
            }
        }
    }
    error_rate.to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let field_count = input.fields.len();
    let mut column_mask_for_fields:Vec<u32> = vec![(1<<field_count) - 1; field_count];
    for ticket in input.other_tickets.iter() {
        let mut is_valid = true;
        for val in ticket.iter() {
            let mut found_valid_field = false;
            for field in input.fields.iter() {
                if (*val >= field.min1 && *val <= field.max1) || (*val >= field.min2 && *val <= field.max2) {
                    found_valid_field = true;
                    break;
                }
            }
            if !found_valid_field {
                //println!("Ticket {:?} is invalid; skipping", ticket);
                is_valid = false;
                break;
            }
        }
        if is_valid {
            // iterate again over valid tickets.
            // TODO: Could store these masks the first time around if needed.
            //println!("Checking ticket {:?}...", ticket);
            for (i_val,val) in ticket.iter().enumerate() {
                for (i_field,field) in input.fields.iter().enumerate() {
                    if (*val >= field.min1 && *val <= field.max1) || (*val >= field.min2 && *val <= field.max2) {
                    } else {
                        // The value in this column failed the check for field i_field,
                        // so field i_field can't be stored in column i_val
                        column_mask_for_fields[i_field] &= !(1<<i_val);
                        //println!("{} is invalid for field {}; mask={:b}", val, i_field, column_mask_for_fields[i_field]);
                    }
                }
            }
        }
    }
    //println!("column_mask_for_fields: {:X?}", column_mask_for_fields);

    let mut solved_mask = 0;
    loop {
        let mut new_solved_mask = 0;
        for mask in column_mask_for_fields.iter() {
            if mask.count_ones() == 1 && (mask & solved_mask) == 0 {
                new_solved_mask = *mask;
                solved_mask |= mask;
                break;
            }
        }
        if new_solved_mask == 0 {
            break;
        }
        for mask in column_mask_for_fields.iter_mut() {
            if *mask != new_solved_mask {
                *mask &= !new_solved_mask;
            }
        }
    }
    let column_for_fields:Vec<usize> = column_mask_for_fields.iter().map(|m| m.trailing_zeros() as usize).collect();
    //println!("column_for_fields: {:?}", column_for_fields);
    let mut prod = 1u64;
    for (i_field, field) in input.fields.iter().enumerate() {
        let col = column_for_fields[i_field];
        let val = input.your_ticket[col];
        assert!((val >= field.min1 && val <= field.max1) || (val >= field.min2 && val <= field.max2));
        if field.name.starts_with("departure") {
            //println!("Multiplying {} (ticket[{}]={}) into prod", field.name, col, val);
            prod *= val as u64;
        }
    }
    prod.to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    let field_re = Regex::new(
        r"^(?P<name>[a-z ]+):\s+(?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)$",
    )
    .unwrap();
    let mut fields = Vec::new();
    let mut mode = 1;
    let mut your_ticket = Vec::new();
    let mut other_tickets = Vec::new();
    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        } else if mode == 1 {
            if line == "your ticket:" {
                mode = 2;
                continue;
            }
            let field_caps = field_re.captures(line).unwrap();
            let name = field_caps.name("name").unwrap().as_str();
            let min1 = field_caps
                .name("min1")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let max1 = field_caps
                .name("max1")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let min2 = field_caps
                .name("min2")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let max2 = field_caps
                .name("max2")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            fields.push(Field {
                name,
                min1,
                max1,
                min2,
                max2,
            });
        } else if mode == 2 {
            if line == "nearby tickets:" {
                mode = 3;
                continue;
            }
            let ticket: Vec<u32> = line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            assert_eq!(fields.len(), ticket.len());
            your_ticket = ticket;
        } else if mode == 3 {
            let ticket: Vec<u32> = line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            assert_eq!(fields.len(), ticket.len());
            other_tickets.push(ticket);
        }
    }
    Input {
        fields,
        your_ticket,
        other_tickets,
    }
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
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

#[test]
fn test_day16_part1() {
    process_text(_TEST_INPUT1, solve_part1, "71");
}

const _TEST_INPUT2: &str = "\
departure class: 0-1 or 4-19
row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
25,7,1
3,9,18
15,1,5
5,14,9";

#[test]
fn test_day16_part2() {
    process_text(_TEST_INPUT2, solve_part2, "156");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input16.txt", solve_part1, "22073")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input16.txt", solve_part2, "1346570764607")
    );
}
