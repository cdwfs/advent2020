use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Input<'a> {
    equations:Vec<&'a str>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn eval_expr_no_parens1(expr:&str) -> i64 {
    let mut result = 0;
    let mut op = "+";
    for tok in expr.split(' ') {
        if tok == "+" || tok == "*" {
            op = tok;
        } else {
            let num = tok.parse::<i64>().unwrap();
            if op == "+" {
                result += num;
            } else if op == "*" {
                result *= num;
            } else {
            }
        }
    }
    result
}

fn eval_expr1(expr:&str) -> i64 {
    let mut e = expr.to_string();
    let lparen_indices:Vec<usize> = e.rmatch_indices('(').map(|(i,_)| i).collect();
    for lparen_index in lparen_indices {
        let rparen_index = lparen_index + e[lparen_index..].find(')').unwrap();
        let result = eval_expr_no_parens1(&e[lparen_index+1..rparen_index]);
        e = format!("{}{}{}", e[0..lparen_index].to_string(), result, e[rparen_index+1..].to_string());
    }
    eval_expr_no_parens1(&e)
}

fn eval_expr_no_parens2(expr:&str) -> i64 {
    let tokens:Vec<&str> = expr.split(' ').collect();
    let mut tokens2:Vec<String> = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "+" {
            let a = tokens2.pop().unwrap().parse::<i64>().unwrap();
            let b = tokens[i+1].parse::<i64>().unwrap();
            tokens2.push((a+b).to_string());
            i += 2;
        } else {
            tokens2.push(tokens[i].to_string());
            i += 1;
        }
    }
    let mut result = 0;
    let mut op = "+";
    for tok in &tokens2 {
        if tok == "*" {
            op = tok;
        } else {
            let num = tok.parse::<i64>().unwrap();
            if op == "+" {
                result += num;
            } else if op == "*" {
                result *= num;
            } else {
            }
        }
    }
    result
}

fn eval_expr2(expr:&str) -> i64 {
    let mut e = expr.to_string();
    let lparen_indices:Vec<usize> = e.rmatch_indices('(').map(|(i,_)| i).collect();
    for lparen_index in lparen_indices {
        let rparen_index = lparen_index + e[lparen_index..].find(')').unwrap();
        let result = eval_expr_no_parens2(&e[lparen_index+1..rparen_index]);
        e = format!("{}{}{}", e[0..lparen_index].to_string(), result, e[rparen_index+1..].to_string());
    }
    eval_expr_no_parens2(&e)
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let mut sum = 0;
    for equation in &input.equations {
        sum += eval_expr1(equation);
    }
    sum.to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let mut sum = 0;
    for equation in &input.equations {
        sum += eval_expr2(equation);
    }
    sum.to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    Input { equations: input_text.lines().collect() }
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
fn test_day18_part1() {
    process_text("1 + 2 * 3 + 4 * 5 + 6", solve_part1, "71");
    process_text("1 + (2 * 3) + (4 * (5 + 6))", solve_part1, "51");
    process_text("2 * 3 + (4 * 5)", solve_part1, "26");
    process_text("5 + (8 * 3 + 9 + 3 * 4 * 3)", solve_part1, "437");
    process_text("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", solve_part1, "12240");
    process_text("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", solve_part1, "13632");
}

#[test]
fn test_day18_part2() {
    process_text("1 + 2 * 3 + 4 * 5 + 6", solve_part2, "231");
    process_text("1 + (2 * 3) + (4 * (5 + 6))", solve_part2, "51");
    process_text("2 * 3 + (4 * 5)", solve_part2, "46");
    process_text("5 + (8 * 3 + 9 + 3 * 4 * 3)", solve_part2, "1445");
    process_text("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", solve_part2, "669060");
    process_text("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", solve_part2, "23340");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input18.txt", solve_part1, "31142189909908")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input18.txt", solve_part2, "323912478287549")
    );
}
