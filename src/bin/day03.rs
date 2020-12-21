use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct Grid {
    width: usize,
    height: usize,
    elems: Vec<u8>,
}

impl Grid {
    fn get(self: &Grid, x: usize, y: usize) -> u8 {
        // In this problem, the grid repeats endlessly in the X dimension
        self.elems[y * self.width + (x % self.width)]
    }
    fn set(self: &mut Grid, x: usize, y: usize, val: u8) {
        self.elems[y * self.width + x] = val;
    }
    fn new(width: usize, height: usize, val: u8) -> Grid {
        Grid {
            width,
            height,
            elems: vec![val; width * height],
        }
    }
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Grid) -> String;

// concrete instance of a ProcessInputFunc implementation
fn count_tree_hits(grid: &Grid, dx: usize, dy: usize) -> usize {
    assert_ne!(grid.get(0, 0), b'#');
    let mut x = 0;
    let mut y = 0;
    let mut hit_count = 0;
    while y < grid.height {
        if grid.get(x, y) == b'#' {
            hit_count += 1;
        }
        x += dx;
        y += dy;
    }
    hit_count
}

fn solve_part1(grid: &Grid) -> String {
    count_tree_hits(grid, 3, 1).to_string()
}

fn solve_part2(grid: &Grid) -> String {
    let prod = count_tree_hits(grid, 1, 1)
        * count_tree_hits(grid, 3, 1)
        * count_tree_hits(grid, 5, 1)
        * count_tree_hits(grid, 7, 1)
        * count_tree_hits(grid, 1, 2);
    prod.to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input: &str) -> Grid {
    let mut grid_width = 0;
    let mut grid_height = 0;
    for line in input.lines() {
        grid_width = line.as_bytes().len();
        grid_height += 1;
    }
    let mut grid = Grid::new(grid_width, grid_height, b'.');
    for (y, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        for (x, b) in bytes.iter().enumerate() {
            if *b == b'#' {
                grid.set(x, y, b'#');
            }
        }
    }
    grid
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
fn test_day03_part1() {
    let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    process_text(input, solve_part1, "7");
}

#[test]
fn test_day03_part2() {
    let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    process_text(input, solve_part2, "336");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input03.txt", solve_part1, "178")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input03.txt", solve_part2, "3492520200")
    );
}
