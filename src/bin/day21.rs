use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}
#[derive(Debug)]
struct Input<'a> {
    foods: Vec<Food<'a>>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    // Collect possible ingredients for each allergen
    let mut allergen_candidates:HashMap<&str,HashSet<&str>> = HashMap::new();
    let mut non_allergenic_ingredients:HashSet<&str> = HashSet::new();
    input.foods.iter().for_each(|food| {
        for ingredient in food.ingredients.iter() {
            let _ = non_allergenic_ingredients.insert(ingredient);
        }
        food.allergens.iter().for_each(|allergen| {
            let candidates = allergen_candidates.entry(allergen).or_insert(food.ingredients.clone());
            // TODO: why doesn't *candidates = candidates.intersection(&food.ingredients).collect() work here?
            let mut new_candidates:HashSet<&str> = HashSet::with_capacity(candidates.len());
            candidates.intersection(&food.ingredients).for_each(|ing| { let _ = new_candidates.insert(ing); });
            *candidates = new_candidates;
        });
    });
    // Find allergens with only one ingredient
    let mut solved = Vec::new();
    while !allergen_candidates.is_empty() {
        let mut new_solved = Vec::new();
        for (allergen,candidates) in allergen_candidates.iter() {
            if candidates.len() == 1 {
                // again, can't find an easy way to gather the set elements into an indexable collection, so...
                for ingredient in candidates.iter() {
                    new_solved.push((*allergen,*ingredient));
                }
            }
        }
        // Remove all newly-solved allergens from the unsolved map
        for (allergen,ingredient) in new_solved.iter() {
            let _ = allergen_candidates.remove(*allergen);
            solved.push((*allergen,*ingredient));
            non_allergenic_ingredients.remove(*ingredient);
        }
        // removed all newly-solved ingredients from the ingredient map
        for (_,ingredient) in new_solved.iter() {
            for (_,candidates) in allergen_candidates.iter_mut() {
                let _ = candidates.remove(*ingredient);
            }
        }
    }
    // Count occurences of non-allergenic ingredients in all foods
    let mut count = 0;
    for food in input.foods.iter() {
        for ingredient in food.ingredients.iter() {
            if non_allergenic_ingredients.contains(ingredient) {
                count += 1;
            }
        }
    }
    count.to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    // Collect possible ingredients for each allergen
    let mut allergen_candidates:HashMap<&str,HashSet<&str>> = HashMap::new();
    let mut non_allergenic_ingredients:HashSet<&str> = HashSet::new();
    input.foods.iter().for_each(|food| {
        for ingredient in food.ingredients.iter() {
            let _ = non_allergenic_ingredients.insert(ingredient);
        }
        food.allergens.iter().for_each(|allergen| {
            let candidates = allergen_candidates.entry(allergen).or_insert(food.ingredients.clone());
            // TODO: why doesn't *candidates = candidates.intersection(&food.ingredients).collect() work here?
            let mut new_candidates:HashSet<&str> = HashSet::with_capacity(candidates.len());
            candidates.intersection(&food.ingredients).for_each(|ing| { let _ = new_candidates.insert(ing); });
            *candidates = new_candidates;
        });
    });
    // Find allergens with only one ingredient
    let mut solved = Vec::new();
    while !allergen_candidates.is_empty() {
        let mut new_solved = Vec::new();
        for (allergen,candidates) in allergen_candidates.iter() {
            if candidates.len() == 1 {
                // again, can't find an easy way to gather the set elements into an indexable collection, so...
                for ingredient in candidates.iter() {
                    new_solved.push((*allergen,*ingredient));
                }
            }
        }
        // Remove all newly-solved allergens from the unsolved map
        for (allergen,ingredient) in new_solved.iter() {
            let _ = allergen_candidates.remove(*allergen);
            solved.push((*allergen,*ingredient));
            non_allergenic_ingredients.remove(*ingredient);
        }
        // removed all newly-solved ingredients from the ingredient map
        for (_,ingredient) in new_solved.iter() {
            for (_,candidates) in allergen_candidates.iter_mut() {
                let _ = candidates.remove(*ingredient);
            }
        }
    }
    solved.sort_by(|(all0,_),(all1,_)| all0.partial_cmp(all1).unwrap());
    let mut dangerous_ingredients = Vec::with_capacity(solved.len());
    for (_,ingredient) in solved.iter() {
        dangerous_ingredients.push(*ingredient);
    }
    dangerous_ingredients.join(",")
}
// Day-specific code to process text data into custom problem state
#[rustfmt::skip]
fn parse_input_text(input_text: &str) -> Input {
    let line_re = Regex::new(r"^(?P<ingredients>[a-z ]+) \(contains (?P<allergens>[a-z, ]+)\)$").unwrap();
    Input {
        foods: input_text.lines().map(|line| {
            let caps = line_re.captures(line).unwrap();
            Food {
                ingredients: caps.name("ingredients").unwrap().as_str().split(' ').collect(),
                allergens: caps.name("allergens").unwrap().as_str().split(", ").collect(),
            }
        }).collect(),
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
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

#[test]
fn test_day21_part1() {
    process_text(_TEST_INPUT1, solve_part1, "5");
}

#[test]
fn test_day21_part2() {
    process_text(_TEST_INPUT1, solve_part2, "mxmxvkd,sqjhc,fvjkl");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input21.txt", solve_part1, "1885")
    );
    println!(
        "Part 2: {}",
        process_file(
            "inputs/input21.txt",
            solve_part2,
            "fllssz,kgbzf,zcdcdf,pzmg,kpsdtv,fvvrc,dqbjj,qpxhfp"
        )
    );
}
