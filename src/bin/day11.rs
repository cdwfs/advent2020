use std::convert::TryFrom;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
struct Input {
    width: i32,
    height: i32,
    grid_old: Vec<u8>,
    grid_new: Vec<u8>,
}

impl Input {
    fn is_valid(self: &Input, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    fn get_old(self: &Input, x: i32, y: i32) -> Option<u8> {
        match self.is_valid(x, y) {
            true => Some(self.grid_old[usize::try_from(y * self.width + x).unwrap()]),
            false => None,
        }
    }
    fn set_new(self: &mut Input, x: i32, y: i32, val: u8) {
        match self.is_valid(x, y) {
            true => self.grid_new[usize::try_from(y * self.width + x).unwrap()] = val,
            false => {}
        }
    }
    fn swap(self: &mut Input) {
        std::mem::swap(&mut self.grid_old, &mut self.grid_new);
    }
    fn is_stable(self: &Input) -> bool {
        for i in 0..self.grid_new.len() {
            if self.grid_new[i] != self.grid_old[i] {
                return false;
            }
        }
        true
    }
    fn num_occupied_neighbors(self: &Input, x: i32, y: i32) -> u32 {
        let mut count = 0;
        for ny in y - 1..y + 2 {
            for nx in x - 1..x + 2 {
                if x != nx || y != ny {
                    if let Some(b'#') = self.get_old(nx, ny) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    #[rustfmt::skip]
    fn num_occupied_visible_neighbors(self:&Input, x:i32, y:i32) -> u32 {
        let mut count = 0;
        // NW
        let mut nx = x-1;
        let mut ny = y-1;
        loop {
            match self.get_old(nx,ny) {
                Some(b'#') => { count += 1; break; }
                Some(b'L') | None => { break ;}
                Some(b'.') => { nx -= 1; ny -= 1; }
                _ => {panic!("Unexpected char");}
            }
        }
        // N
        let nx = x;
        let mut ny = y-1;
        loop {
            match self.get_old(nx,ny) {
                Some(b'#') => { count += 1; break; }
                Some(b'L') | None => { break ;}
                Some(b'.') => { ny -= 1; }
                _ => {panic!("Unexpected char");}
            }
        }
        // NE
        let mut nx = x+1;
        let mut ny = y-1;
        loop {
            match self.get_old(nx,ny) {
                Some(b'#') => { count += 1; break; }
                Some(b'L') | None => { break ;}
                Some(b'.') => { nx += 1; ny -= 1; }
                _ => {panic!("Unexpected char");}
            }
        }
        // W
        let mut nx = x-1;
        let ny = y;
        loop {
            match self.get_old(nx,ny) {
                Some(b'#') => { count += 1; break; }
                Some(b'L') | None => { break ;}
                Some(b'.') => { nx -= 1; }
                _ => {panic!("Unexpected char");}
            }
        }
        // E
        let mut nx = x+1;
        let ny = y;
        loop {
            match self.get_old(nx,ny) {
                Some(b'#') => { count += 1; break; }
                Some(b'L') | None => { break ;}
                Some(b'.') => { nx += 1; }
                _ => {panic!("Unexpected char");}
            }
        }
        // SW
        let mut nx = x-1;
        let mut ny = y+1;
        loop {
            match self.get_old(nx,ny) {
                Some(b'#') => { count += 1; break; }
                Some(b'L') | None => { break ;}
                Some(b'.') => { nx -= 1; ny += 1; }
                _ => {panic!("Unexpected char");}
            }
        }
        // S
        let nx = x;
        let mut ny = y+1;
        loop {
            match self.get_old(nx,ny) {
                Some(b'#') => { count += 1; break; }
                Some(b'L') | None => { break ;}
                Some(b'.') => { ny += 1; }
                _ => {panic!("Unexpected char");}
            }
        }
        // SE
        let mut nx = x+1;
        let mut ny = y+1;
        loop {
            match self.get_old(nx,ny) {
                Some(b'#') => { count += 1; break; }
                Some(b'L') | None => { break ;}
                Some(b'.') => { nx += 1; ny += 1; }
                _ => {panic!("Unexpected char");}
            }
        }
        count
    }
    fn num_occupied(self: &Input) -> u32 {
        // TODO(cort): functional nonsense?
        let mut count = 0;
        for &b in self.grid_old.iter() {
            if b == b'#' {
                count += 1;
            }
        }
        count
    }
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&mut Input) -> String;

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &mut Input) -> String {
    loop {
        for y in 0..input.height {
            for x in 0..input.width {
                input.set_new(x, y, match input.get_old(x,y) {
                    Some(b'L') => if input.num_occupied_neighbors(x,y) == 0 { b'#' } else { b'L' },
                    Some(b'#') => if input.num_occupied_neighbors(x,y) >= 4 { b'L' } else { b'#' },
                    Some(b'.') => b'.',
                    _ => b'?',
                });
            }
        }
        if input.is_stable() {
            break;
        }
        input.swap();
    }
    input.num_occupied().to_string()
}

#[rustfmt::skip]
fn solve_part2(input: &mut Input) -> String {
    loop {
        for y in 0..input.height {
            for x in 0..input.width {
                input.set_new(x, y, match input.get_old(x,y) {
                    Some(b'L') => if input.num_occupied_visible_neighbors(x,y) == 0 { b'#' } else { b'L' },
                    Some(b'#') => if input.num_occupied_visible_neighbors(x,y) >= 5 { b'L' } else { b'#' },
                    Some(b'.') => b'.',
                    _ => b'?',
                });
            }
        }
        if input.is_stable() {
            break;
        }
        input.swap();
    }
    input.num_occupied().to_string()
}

// Day-specific code to process text data into custom problem state
fn parse_input_text(input_text: &str) -> Input {
    let lines: Vec<&str> = input_text.lines().collect();
    let width = lines.first().unwrap().len() as i32;
    let height = lines.len() as i32;
    let grid = lines.join("").as_bytes().to_vec();
    let grid_len = grid.len();
    Input {
        width,
        height,
        grid_old: grid,
        grid_new: vec![b'.'; grid_len],
    }
}

fn process_text(input_text: &str, processor: ProcessInputFunc, expected: &str) -> String {
    let mut state = parse_input_text(input_text);
    let actual = processor(&mut state);
    assert_eq!(expected, actual);
    actual
}

fn process_file(filename: &str, processor: ProcessInputFunc, expected: &str) -> String {
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Could not load {}", filename));
    process_text(&contents, processor, expected)
}

const _TEST_INPUT1: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

#[test]
fn test_day11_part1() {
    process_text(_TEST_INPUT1, solve_part1, "37");
}

#[test]
fn test_day11_part2() {
    process_text(_TEST_INPUT1, solve_part2, "26");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input11.txt", solve_part1, "2303")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input11.txt", solve_part2, "2057")
    );
}
