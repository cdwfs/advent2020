use std::collections::HashSet;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Clone, Copy, Debug)]
enum HexDir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}
#[derive(Debug)]
struct Input {
    paths: Vec<Vec<HexDir>>,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

fn get_black_tiles(input: &Input) -> HashSet<(i16, i16)> {
    let mut black_tiles = HashSet::with_capacity(input.paths.len());
    for path in input.paths.iter() {
        // axial coordinates! https://www.redblobgames.com/grids/hexagons/#coordinates
        let mut tile = (0, 0);
        for dir in path.iter() {
            tile = match *dir {
                HexDir::E => (tile.0 + 1, tile.1),
                HexDir::W => (tile.0 - 1, tile.1),
                HexDir::NE => (tile.0 + 1, tile.1 - 1),
                HexDir::NW => (tile.0, tile.1 - 1),
                HexDir::SE => (tile.0, tile.1 + 1),
                HexDir::SW => (tile.0 - 1, tile.1 + 1),
            }
        }
        if black_tiles.contains(&tile) {
            black_tiles.remove(&tile);
        } else {
            black_tiles.insert(tile);
        }
    }
    black_tiles
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    get_black_tiles(input).len().to_string()
}

fn tile_neighbors(tile: &(i16, i16)) -> [(i16, i16); 6] {
    [
        (tile.0 + 1, tile.1),     // E
        (tile.0 - 1, tile.1),     // W
        (tile.0 + 1, tile.1 - 1), // NE
        (tile.0, tile.1 - 1),     // NW
        (tile.0, tile.1 + 1),     // SE
        (tile.0 - 1, tile.1 + 1), // SW
    ]
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let mut black_tiles = get_black_tiles(input);
    let mut candidates = HashSet::with_capacity(black_tiles.len() * 7);
    for tile in black_tiles.iter() {
        let _ = candidates.insert((tile.0,  tile.1)  ); // self
        for neighbor in tile_neighbors(tile).iter() {
            let _ = candidates.insert(*neighbor);
        }
    }
    for _ in 1..101 {
        let mut new_black_tiles = HashSet::with_capacity(candidates.len());
        let mut new_candidates = HashSet::with_capacity(candidates.len() * 7);
        for tile in candidates.iter() {
            let mut num_black_neighbors = 0;
            let neighbors = tile_neighbors(tile);
            for neighbor in neighbors.iter() {
                if black_tiles.contains(&neighbor) {
                    num_black_neighbors += 1;
                    if num_black_neighbors >= 3 {
                        break;
                    }
                }
            }
            
            if black_tiles.contains(&tile) {
                // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
                if num_black_neighbors == 0 || num_black_neighbors > 2 {
                    // nothing to do, tile is flipped to white
                } else {
                    // tile stays black
                    let _ = new_black_tiles.insert(*tile);
                    let _ = new_candidates.insert(*tile);
                    for neighbor in neighbors.iter() {
                        let _ = new_candidates.insert(*neighbor);
                    }
                }
            } else if num_black_neighbors == 2 {
            // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
                let _ = new_black_tiles.insert(*tile);
                let _ = new_candidates.insert(*tile);
                for neighbor in neighbors.iter() {
                    let _ = new_candidates.insert(*neighbor);
                }
            }
        }
        black_tiles = new_black_tiles;
        candidates = new_candidates;
    }
    black_tiles.len().to_string()
}

// Day-specific code to process text data into custom problem state
#[rustfmt::skip]
fn parse_input_text(input_text: &str) -> Input {
    Input {
        paths: input_text.lines().map(|line| {
            let mut dirs = Vec::new();
            let mut itor = line.bytes();
            while let Some(b) = itor.next() {
                dirs.push(match b {
                    b'e' => HexDir::E,
                    b'w' => HexDir::W,
                    b'n' => if itor.next().unwrap() == b'e' {HexDir::NE} else {HexDir::NW},
                    b's' => if itor.next().unwrap() == b'e' {HexDir::SE} else {HexDir::SW},
                    _ => panic!("Invalid direction prefix {}", b)
                });
            }
            dirs
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
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

#[test]
fn test_day24_part1() {
    process_text(_TEST_INPUT1, solve_part1, "10");
}

#[test]
fn test_day24_part2() {
    process_text(_TEST_INPUT1, solve_part2, "2208");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input24.txt", solve_part1, "497")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input24.txt", solve_part2, "4156")
    );
}
