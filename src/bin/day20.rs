use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs;

// TODO: a trait, to mark this as "Thing That Is The Result Of Processing Input"
#[derive(Debug)]
struct Tile {
    pixels: [u8; 100],
    id: usize,
    edges: [u16; 8], // N,E,S,W,Nf,Ef,Sf,W,
}
#[allow(clippy::erasing_op, clippy::identity_op)]
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id).unwrap();
        for y in 0..10 {
            writeln!(
                f,
                "{}{}{}{}{}{}{}{}{}{}",
                self.pixels[10 * y + 0] as char,
                self.pixels[10 * y + 1] as char,
                self.pixels[10 * y + 2] as char,
                self.pixels[10 * y + 3] as char,
                self.pixels[10 * y + 4] as char,
                self.pixels[10 * y + 5] as char,
                self.pixels[10 * y + 6] as char,
                self.pixels[10 * y + 7] as char,
                self.pixels[10 * y + 8] as char,
                self.pixels[10 * y + 9] as char
            )
            .unwrap();
        }
        fmt::Result::Ok(())
    }
}
#[derive(Debug)]
struct TileInGrid {
    id: usize,
    right_rev_mask: u16,
    down_rev_mask: u16,
}
#[derive(Debug)]
struct Input {
    tiles: Vec<Tile>,
    dim: usize,
}

// Generic signature for "process problem state to get an answer"
type ProcessInputFunc = fn(&Input) -> String;

const SIDE_LUT: [usize; 8] = [6, 5, 4, 7, 2, 1, 0, 3];
const ROTR_LUT: [usize; 8] = [1, 2, 3, 0, 5, 6, 7, 4];

fn find_next_tile(
    grid: &mut Vec<TileInGrid>,
    dim: usize,
    unused_tile_ids: Vec<usize>,
    tiles: &HashMap<usize, &Tile>,
) -> bool {
    if unused_tile_ids.is_empty() {
        return true;
    }
    let ty = grid.len() / dim;
    let tx = grid.len() % dim;
    for id in unused_tile_ids.iter() {
        let tile = tiles.get(id).unwrap();
        for left_face in 0..8 {
            let up_face = ROTR_LUT[left_face];
            let mut is_match = false;
            if tx > 0 && ty > 0 {
                // interior tiles need to match both the tile above and to their left
                let up_tile_mask = grid[dim * (ty - 1) + tx].down_rev_mask;
                let left_tile_mask = grid[dim * ty + tx - 1].right_rev_mask;
                if tile.edges[left_face] == left_tile_mask && tile.edges[up_face] == up_tile_mask {
                    is_match = true;
                }
            } else if tx == 0 && ty > 0 {
                // left column of tiles only needs to match above them
                let up_tile_mask = grid[dim * (ty - 1) + tx].down_rev_mask;
                if tile.edges[up_face] == up_tile_mask {
                    is_match = true;
                }
            } else if tx > 0 && ty == 0 {
                // top row of tiles only needs to match to the left
                let left_tile_mask = grid[dim * ty + tx - 1].right_rev_mask;
                if tile.edges[left_face] == left_tile_mask {
                    is_match = true;
                }
            } else if tx == 0 && ty == 0 {
                // upper-left tile is unconstrained
                is_match = true;
            }

            // If this orientation of this tile meets the criteria, speculatively add it to the grid and recurse
            if is_match {
                let right_rev_mask = tile.edges[SIDE_LUT[left_face]];
                let down_rev_mask = tile.edges[SIDE_LUT[up_face]];
                grid.push(TileInGrid {
                    id: *id,
                    right_rev_mask,
                    down_rev_mask,
                });
                let unused_tile_ids: Vec<usize> = unused_tile_ids
                    .iter()
                    .filter_map(|tile_id| {
                        if *tile_id != *id {
                            Some(*tile_id)
                        } else {
                            None
                        }
                    })
                    .collect();
                if find_next_tile(grid, dim, unused_tile_ids, tiles) {
                    return true;
                }
                let _ = grid.pop();
            }
        }
    }
    false
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    //for tile in &input.tiles {
    //    println!("{}", tile);
    //}
    let dim = input.dim;
    let mut all_tiles:HashMap<usize,&Tile> = HashMap::with_capacity(input.tiles.len());
    let mut unused_tile_ids = Vec::with_capacity(input.tiles.len());
    for tile in input.tiles.iter() {
        unused_tile_ids.push(tile.id);
        all_tiles.insert(tile.id, tile);
    }
    let mut grid:Vec<TileInGrid> = Vec::with_capacity(input.tiles.len());
    let success = find_next_tile(&mut grid, dim, unused_tile_ids, &all_tiles);
    assert!(success);
    let upper_left_id = grid[0].id;
    let upper_right_id = grid[dim-1].id;
    let lower_left_id = grid[dim*(dim-1)].id;
    let lower_right_id = grid[dim*(dim-1)+dim-1].id;
    (upper_left_id * upper_right_id * lower_left_id * lower_right_id).to_string()
}

#[rustfmt::skip]
fn solve_part2(_input: &Input) -> String {
    "TBD".to_string()
}

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
fn make_edge_mask(b0:u8, b1:u8, b2:u8, b3:u8, b4:u8, b5:u8, b6:u8, b7:u8, b8:u8, b9:u8) -> u16 {
    (if b0 == b'#' {1<<9} else {0}) |
    (if b1 == b'#' {1<<8} else {0}) |
    (if b2 == b'#' {1<<7} else {0}) |
    (if b3 == b'#' {1<<6} else {0}) |
    (if b4 == b'#' {1<<5} else {0}) |
    (if b5 == b'#' {1<<4} else {0}) |
    (if b6 == b'#' {1<<3} else {0}) |
    (if b7 == b'#' {1<<2} else {0}) |
    (if b8 == b'#' {1<<1} else {0}) |
    (if b9 == b'#' {1<<0} else {0})
}

#[rustfmt::skip]
fn reverse_mask(mask:u16) -> u16 {
    ((mask << 9) & 0b10_0000_0000) |
    ((mask >> 9) & 0b00_0000_0001) |
    ((mask << 7) & 0b01_0000_0000) |
    ((mask >> 7) & 0b00_0000_0010) |
    ((mask << 5) & 0b00_1000_0000) |
    ((mask >> 5) & 0b00_0000_0100) |
    ((mask << 3) & 0b00_0100_0000) |
    ((mask >> 3) & 0b00_0000_1000) |
    ((mask << 1) & 0b00_0010_0000) |
    ((mask >> 1) & 0b00_0001_0000)
}

// Day-specific code to process text data into custom problem state
#[allow(clippy::identity_op)]
#[rustfmt::skip]
fn parse_input_text(input_text: &str) -> Input {
    let mut tiles = Vec::new();
    let tile_id_re = Regex::new(r"^Tile (?P<id>\d{4}):$").unwrap();
    let mut lines = input_text.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let id = tile_id_re.captures(line).unwrap().name("id").unwrap().as_str();
        let id = id.parse::<usize>().unwrap();
        let mut p = [b'.';100];
        for y in 0..10 {
            let line = lines.next().unwrap();
            let row = line.as_bytes();
            for x in 0..10 {
                p[y*10+x] = row[x];
            }
        }
        let mut edges = [0u16;8];

        #[allow(clippy::erasing_op)]
        #[allow(clippy::identity_op)]
        {
            edges[0] = make_edge_mask(p[10*0+0], p[10*0+1], p[10*0+2], p[10*0+3], p[10*0+4],
                p[10*0+5], p[10*0+6], p[10*0+7], p[10*0+8], p[10*0+9]); // N
            edges[1] = make_edge_mask(p[10*0+9], p[10*1+9], p[10*2+9], p[10*3+9], p[10*4+9],
                p[10*5+9], p[10*6+9], p[10*7+9], p[10*8+9], p[10*9+9]); // E
            edges[2] = make_edge_mask(p[10*9+9], p[10*9+8], p[10*9+7], p[10*9+6], p[10*9+5],
                p[10*9+4], p[10*9+3], p[10*9+2], p[10*9+1], p[10*9+0]); // S
            edges[3] = make_edge_mask(p[10*9+0], p[10*8+0], p[10*7+0], p[10*6+0], p[10*5+0],
                p[10*4+0], p[10*3+0], p[10*2+0], p[10*1+0], p[10*0+0]); // W
            edges[4] = reverse_mask(edges[0]); // N'
            edges[5] = reverse_mask(edges[3]); // E'
            edges[6] = reverse_mask(edges[2]); // S'
            edges[7] = reverse_mask(edges[1]); // W'
        }
        tiles.push(Tile{id,pixels:p,edges});
    }
    let dim = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(tiles.len(), dim*dim); // if this fails, we somehow missed parsing a tile
    Input { tiles, dim }
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
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...

";
#[test]
fn test_day20_part1() {
    assert_eq!(
        make_edge_mask(b'#', b'.', b'#', b'.', b'#', b'.', b'#', b'.', b'#', b'.'),
        0b10_1010_1010
    );
    assert_eq!(
        make_edge_mask(b'.', b'#', b'.', b'#', b'.', b'#', b'.', b'#', b'.', b'#'),
        0b01_0101_0101
    );
    assert_eq!(reverse_mask(0b10_1010_1010), 0b01_0101_0101);
    assert_eq!(reverse_mask(0b11_1110_0000), 0b00_0001_1111);
    process_text(_TEST_INPUT1, solve_part1, "20899048083289");
}

#[test]
fn test_day20_part2() {
    process_text(_TEST_INPUT1, solve_part2, "12");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input20.txt", solve_part1, "15670959891893")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input20.txt", solve_part2, "273")
    );
}
