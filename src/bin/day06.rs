use std::fs;

struct Group {
    members: Vec<String>,
}
// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct GroupList {
    groups: Vec<Group>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&GroupList) -> String;

fn get_group_yes_count_any(group: &Group) -> u32 {
    let mut mask: u32 = 0;
    for member in group.members.iter() {
        for b in member.as_bytes().iter() {
            mask |= 1 << (b - b'a');
        }
    }
    mask.count_ones()
}

fn get_group_yes_count_all(group: &Group) -> u32 {
    let mut group_mask: u32 = 0xFFFF_FFFF;
    for member in group.members.iter() {
        let mut member_mask = 0x0000_0000;
        for b in member.as_bytes().iter() {
            member_mask |= 1 << (b - b'a');
        }
        group_mask &= member_mask;
    }
    group_mask.count_ones()
}

// concrete instance of a ProcessInputFunc implementation
fn solve_part1(list: &GroupList) -> String {
    let mut yes_count = 0;
    for group in list.groups.iter() {
        yes_count += get_group_yes_count_any(&group);
    }
    yes_count.to_string()
}

// concrete instance of a ProcessInputFunc implementation
fn solve_part2(list: &GroupList) -> String {
    let mut yes_count = 0;
    for group in list.groups.iter() {
        yes_count += get_group_yes_count_all(&group);
    }
    yes_count.to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input: &str) -> GroupList {
    let mut glist = GroupList { groups: Vec::new() };
    let mut group = Group {
        members: Vec::new(),
    };
    for line in input.lines() {
        if line.is_empty() {
            glist.groups.push(group);
            group = Group {
                members: Vec::new(),
            };
            continue;
        }
        group.members.push(String::from(line));
    }
    // add final group to list
    if !group.members.is_empty() {
        glist.groups.push(group);
    }
    glist
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

const _TEST_INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

#[test]
fn test_day06_part1() {
    process_text(_TEST_INPUT, solve_part1, "11");
}

#[test]
fn test_day06_part2() {
    process_text(_TEST_INPUT, solve_part2, "6");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input06.txt", solve_part1, "6549")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input06.txt", solve_part2, "3466")
    );
}
