use regex::Regex;
use std::collections::HashMap;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug, Copy, Clone)]
enum Rule {
    Literal(u8),
    Seq1(u32),
    Seq2(u32, u32),
    Seq3(u32, u32, u32),
    Or1(u32, u32),
    Or2(u32, u32, u32, u32),
}
#[derive(Debug)]
struct Input<'a> {
    rules: HashMap<u32, Rule>,
    messages: Vec<&'a str>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn build_regex_str(rule_id: u32, rules: &HashMap<u32, Rule>) -> String {
    match rules.get(&rule_id).unwrap() {
        Rule::Literal(b) => format!("{}", *b as char),
        Rule::Seq1(a) => build_regex_str(*a, rules),
        Rule::Seq2(a, b) => format!(
            "{}{}",
            build_regex_str(*a, rules),
            build_regex_str(*b, rules)
        ),
        Rule::Seq3(a, b, c) => format!(
            "{}{}{}",
            build_regex_str(*a, rules),
            build_regex_str(*b, rules),
            build_regex_str(*c, rules)
        ),
        Rule::Or1(a, b) => format!(
            "(({})|({}))",
            build_regex_str(*a, rules),
            build_regex_str(*b, rules)
        ),
        Rule::Or2(a, b, c, d) => format!(
            "(({}{})|({}{}))",
            build_regex_str(*a, rules),
            build_regex_str(*b, rules),
            build_regex_str(*c, rules),
            build_regex_str(*d, rules)
        ),
    }
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let re = format!("^{}$", build_regex_str(0, &input.rules));
    let re = Regex::new(&re).unwrap();
    input.messages.iter().fold(0, |count,msg| count + if re.is_match(msg) {1} else {0}).to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let mut res = Vec::with_capacity(20);
    for i in 0..20 {
        let re = format!("^({}){{{},}}({}){{1,{}}}$",
            build_regex_str(42, &input.rules),i+2,
            build_regex_str(31, &input.rules),i+1);
        res.push(Regex::new(&re).unwrap());
    }
    input.messages.iter().fold(0, |count,msg| count + if res.iter().any(|re| re.is_match(msg)) {1} else {0}).to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    let rule_literal_re = Regex::new(r#"^(?P<index>\d+): "(?P<str>[ab])"$"#).unwrap();
    let rule_seq1_re = Regex::new(r"^(?P<index>\d+): (?P<ruleA>\d+)$").unwrap();
    let rule_seq2_re = Regex::new(r"^(?P<index>\d+): (?P<ruleA>\d+) (?P<ruleB>\d+)$").unwrap();
    let rule_seq3_re =
        Regex::new(r"^(?P<index>\d+): (?P<ruleA>\d+) (?P<ruleB>\d+) (?P<ruleC>\d+)$").unwrap();
    let rule_or1_re = Regex::new(r"^(?P<index>\d+): (?P<ruleA>\d+) \| (?P<ruleB>\d+)$").unwrap();
    let rule_or2_re = Regex::new(
        r"^(?P<index>\d+): (?P<ruleA>\d+) (?P<ruleB>\d+) \| (?P<ruleC>\d+) (?P<ruleD>\d+)$",
    )
    .unwrap();
    let message_re = Regex::new(r"^[ab]+$").unwrap();
    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        } else if message_re.is_match(line) {
            messages.push(line);
        } else if let Some(caps) = rule_literal_re.captures(line) {
            let index = caps.name("index").unwrap().as_str().parse::<u32>().unwrap();
            let c = caps.name("str").unwrap().as_str().as_bytes()[0];
            rules.insert(index, Rule::Literal(c));
        } else if let Some(caps) = rule_seq1_re.captures(line) {
            let index = caps.name("index").unwrap().as_str().parse::<u32>().unwrap();
            let a = caps.name("ruleA").unwrap().as_str().parse::<u32>().unwrap();
            rules.insert(index, Rule::Seq1(a));
        } else if let Some(caps) = rule_seq2_re.captures(line) {
            let index = caps.name("index").unwrap().as_str().parse::<u32>().unwrap();
            let a = caps.name("ruleA").unwrap().as_str().parse::<u32>().unwrap();
            let b = caps.name("ruleB").unwrap().as_str().parse::<u32>().unwrap();
            rules.insert(index, Rule::Seq2(a, b));
        } else if let Some(caps) = rule_seq3_re.captures(line) {
            let index = caps.name("index").unwrap().as_str().parse::<u32>().unwrap();
            let a = caps.name("ruleA").unwrap().as_str().parse::<u32>().unwrap();
            let b = caps.name("ruleB").unwrap().as_str().parse::<u32>().unwrap();
            let c = caps.name("ruleC").unwrap().as_str().parse::<u32>().unwrap();
            rules.insert(index, Rule::Seq3(a, b, c));
        } else if let Some(caps) = rule_or1_re.captures(line) {
            let index = caps.name("index").unwrap().as_str().parse::<u32>().unwrap();
            let a = caps.name("ruleA").unwrap().as_str().parse::<u32>().unwrap();
            let b = caps.name("ruleB").unwrap().as_str().parse::<u32>().unwrap();
            rules.insert(index, Rule::Or1(a, b));
        } else if let Some(caps) = rule_or2_re.captures(line) {
            let index = caps.name("index").unwrap().as_str().parse::<u32>().unwrap();
            let a = caps.name("ruleA").unwrap().as_str().parse::<u32>().unwrap();
            let b = caps.name("ruleB").unwrap().as_str().parse::<u32>().unwrap();
            let c = caps.name("ruleC").unwrap().as_str().parse::<u32>().unwrap();
            let d = caps.name("ruleD").unwrap().as_str().parse::<u32>().unwrap();
            rules.insert(index, Rule::Or2(a, b, c, d));
        } else {
            panic!("unmatched line {}", line);
        }
    }
    Input { rules, messages }
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
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

const _TEST_INPUT2: &str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

#[test]
fn test_day19_part1() {
    process_text(_TEST_INPUT1, solve_part1, "2");
    process_text(_TEST_INPUT2, solve_part1, "3");
}

#[test]
fn test_day19_part2() {
    process_text(_TEST_INPUT2, solve_part2, "12");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input19.txt", solve_part1, "180")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input19.txt", solve_part2, "323")
    );
}
