use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct BagRules {
    rules: HashMap<String, Vec<BagTypeAndCount>>,
}
struct BagTypeAndCount {
    bag_type: String,
    count: u32,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&BagRules) -> String;

// concrete instance of a ProcessInputFunc implementation
fn solve_part1(bag_rules: &BagRules) -> String {
    // Build reverse lookup
    let mut held_by = HashMap::<&str, Vec<&str>>::new();
    for (holder, contents) in &bag_rules.rules {
        for content in contents {
            let v = held_by.entry(&content.bag_type).or_insert(Vec::new());
            v.push(holder);
        }
    }
    // Recursive search for bags that can hold "shiny gold"
    let mut to_search = vec!["shiny gold"];
    let mut visited_bags = HashSet::<&str>::new();
    while !to_search.is_empty() {
        let bag = to_search.pop().unwrap();
        if visited_bags.contains(bag) {
            continue;
        }
        visited_bags.insert(&bag);
        if held_by.contains_key(&bag) {
            for holder in held_by.get(&bag).unwrap() {
                to_search.push(holder);
            }
        }
    }
    let len = visited_bags.len() - 1; // subtract one for "shiny gold" itself
    len.to_string()
}

// concrete instance of a ProcessInputFunc implementation
fn get_bag_held_count(
    bag: &str,
    bag_rules: &BagRules,
    held_counts: &mut HashMap<String, u32>,
) -> u32 {
    if held_counts.contains_key(bag) {
        return held_counts[bag];
    }
    let mut count = 0;
    for btac in &bag_rules.rules[bag] {
        count += btac.count * (1 + get_bag_held_count(&btac.bag_type, bag_rules, held_counts));
    }
    held_counts.insert(bag.to_string(), count);
    count
}
fn solve_part2(bag_rules: &BagRules) -> String {
    let mut held_counts = HashMap::new();
    get_bag_held_count("shiny gold", bag_rules, &mut held_counts).to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input: &str) -> BagRules {
    let mut bag_rules = BagRules {
        rules: HashMap::new(),
    };
    let line_re = Regex::new(r"^(?P<holder>[a-z ]+) bags contain (?P<contents>.+)\.$").unwrap();
    let content_re = Regex::new(r"^(?P<count>[\d+]) (?P<type>[a-z ]+) bag(s?)$").unwrap();
    for line in input.lines() {
        let line_caps = line_re.captures(line).unwrap();
        let holder = line_caps.name("holder").unwrap().as_str();
        let contents = line_caps.name("contents").unwrap().as_str();
        assert!(!bag_rules.rules.contains_key(holder));
        let mut v = Vec::new();
        if contents == "no other bags" {
            bag_rules.rules.insert(holder.to_string(), v);
            continue;
        };
        for content in contents.split(", ") {
            let content_caps = content_re.captures(content).unwrap();
            v.push(BagTypeAndCount {
                bag_type: content_caps.name("type").unwrap().as_str().to_string(),
                count: content_caps
                    .name("count")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
            });
        }
        bag_rules.rules.insert(holder.to_string(), v);
    }
    bag_rules
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
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

#[test]
fn test_day07_part1() {
    process_text(_TEST_INPUT1, solve_part1, "4");
}

const _TEST_INPUT2: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

#[test]
fn test_day07_part2() {
    process_text(_TEST_INPUT1, solve_part2, "32");
    process_text(_TEST_INPUT2, solve_part2, "126");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input07.txt", solve_part1, "296")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input07.txt", solve_part2, "9339")
    );
}
