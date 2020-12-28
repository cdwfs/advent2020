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
    up_face: usize,
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
const ROTL_LUT: [usize; 8] = [3, 0, 1, 2, 7, 4, 5, 6];

fn find_next_tile(
    grid: &mut Vec<TileInGrid>,
    dim: usize,
    unused_tile_ids: Vec<usize>,
    tile_map: &HashMap<usize, &Tile>,
) -> bool {
    if unused_tile_ids.is_empty() {
        return true;
    }
    let ty = grid.len() / dim;
    let tx = grid.len() % dim;
    for id in unused_tile_ids.iter() {
        let tile = tile_map.get(id).unwrap();
        for up_face in 0..8 {
            let up_face = 7 - up_face;
            let left_face = ROTL_LUT[up_face];
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
                    up_face,
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
                if find_next_tile(grid, dim, unused_tile_ids, tile_map) {
                    return true;
                }
                let _ = grid.pop();
            }
        }
    }
    false
}

fn find_valid_grid(input: &Input) -> Vec<TileInGrid> {
    let dim = input.dim;
    let mut tile_map: HashMap<usize, &Tile> = HashMap::with_capacity(input.tiles.len());
    let mut unused_tile_ids = Vec::with_capacity(input.tiles.len());
    for tile in input.tiles.iter() {
        unused_tile_ids.push(tile.id);
        tile_map.insert(tile.id, tile);
    }
    let mut grid: Vec<TileInGrid> = Vec::with_capacity(input.tiles.len());
    let success = find_next_tile(&mut grid, dim, unused_tile_ids, &tile_map);
    assert!(success);
    grid
}

// concrete instance of a ProcessInputFunc implementation
#[rustfmt::skip]
fn solve_part1(input: &Input) -> String {
    let grid = find_valid_grid(input);
    let dim = input.dim;
    let upper_left_id = grid[0].id;
    let upper_right_id = grid[dim-1].id;
    let lower_left_id = grid[dim*(dim-1)].id;
    let lower_right_id = grid[dim*(dim-1)+dim-1].id;
    (upper_left_id * upper_right_id * lower_left_id * lower_right_id).to_string()
}

fn assemble_image(input: &Input, tile_dim: usize, pix_dim: usize) -> Vec<u8> {
    let grid = find_valid_grid(input);
    // Copy non-border pixels into an image
    let mut tile_map: HashMap<usize, &Tile> = HashMap::with_capacity(input.tiles.len());
    for tile in input.tiles.iter() {
        tile_map.insert(tile.id, tile);
    }
    let mut image: Vec<u8> = vec![b'?'; pix_dim * pix_dim];
    for (i_tile, tig) in grid.iter().enumerate() {
        let ty = i_tile / tile_dim;
        let tx = i_tile % tile_dim;
        let tile_pixels = &tile_map.get(&tig.id).unwrap().pixels;
        let final_pixels = match tig.up_face {
            0 => Vec::from(*tile_pixels),
            1 => rotr90_image(
                &rotr90_image(&rotr90_image(tile_pixels, 10, 10), 10, 10),
                10,
                10,
            ),
            2 => rotr90_image(&rotr90_image(tile_pixels, 10, 10), 10, 10),
            3 => rotr90_image(tile_pixels, 10, 10),
            4 => fliph_image(tile_pixels, 10, 10),
            5 => fliph_image(&rotr90_image(tile_pixels, 10, 10), 10, 10),
            6 => fliph_image(
                &rotr90_image(&rotr90_image(tile_pixels, 10, 10), 10, 10),
                10,
                10,
            ),
            7 => rotr90_image(&fliph_image(tile_pixels, 10, 10), 10, 10),
            _ => panic!("Invalid up face"),
        };
        for y in 0..8 {
            let dpy = 8 * ty + y;
            let spy = y + 1;
            for x in 0..8 {
                let dpx = 8 * tx + x;
                let spx = x + 1;
                image[pix_dim * dpy + dpx] = final_pixels[10 * spy + spx];
            }
        }
    }
    image
}

// note: output image will have dimensions [height,width]
fn rotr90_image(image: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut output = vec![b'?'; image.len()];
    for sy in 0..height {
        for sx in 0..width {
            output[sx * height + (height - 1 - sy)] = image[sy * width + sx];
        }
    }
    output
}
fn fliph_image(image: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut output = vec![b'?'; image.len()];
    for sy in 0..height {
        for sx in 0..width {
            output[sy * width + (width - 1 - sx)] = image[sy * width + sx];
        }
    }
    output
}

#[rustfmt::skip]
fn solve_part2(input: &Input) -> String {
    let tile_dim = input.dim;
    let pix_dim = tile_dim*(10-2);
    let image = assemble_image(input, tile_dim, pix_dim);
    // Search for sea monsters
    const MONSTER_WIDTH:usize = 20;
    const MONSTER_HEIGHT:usize = 3;
    const MONSTER_OFFSET_COUNT:usize = 15;
    let monster_offsets0:[usize;MONSTER_OFFSET_COUNT] = [18,
        pix_dim, pix_dim+5, pix_dim+6, pix_dim+11, pix_dim+12, pix_dim+17, pix_dim+18, pix_dim+19,
        2*pix_dim+1, 2*pix_dim+4, 2*pix_dim+7, 2*pix_dim+10, 2*pix_dim+13, 2*pix_dim+16];
    let monster_offsets1:[usize;MONSTER_OFFSET_COUNT] = [1, pix_dim,
        4*pix_dim, 5*pix_dim+1, 6*pix_dim+1, 7*pix_dim,
        10*pix_dim, 11*pix_dim+1, 12*pix_dim+1, 13*pix_dim,
        16*pix_dim, 17*pix_dim+1, 18*pix_dim+1, 18*pix_dim+2, 19*pix_dim+1,];
    let monster_offsets2:[usize;MONSTER_OFFSET_COUNT] = [3, 6, 9, 12, 15, 18,
        pix_dim, pix_dim+1, pix_dim+2, pix_dim+7, pix_dim+8, pix_dim+13, pix_dim+14, pix_dim+19,
        2*pix_dim+1];
    let monster_offsets3:[usize;MONSTER_OFFSET_COUNT] = [1, pix_dim, pix_dim+1, 2*pix_dim+1, 3*pix_dim+2,
        6*pix_dim+2, 7*pix_dim+1, 8*pix_dim+1, 9*pix_dim+2,
        12*pix_dim+2, 13*pix_dim+1, 14*pix_dim+1, 15*pix_dim+2,
        18*pix_dim+2, 19*pix_dim+1];
    let monster_offsets4:[usize;MONSTER_OFFSET_COUNT] = [1,
        pix_dim, pix_dim+1, pix_dim+2, pix_dim+7, pix_dim+8, pix_dim+13, pix_dim+14, pix_dim+19,
        2*pix_dim+3, 2*pix_dim+6, 2*pix_dim+9, 2*pix_dim+12, 2*pix_dim+15, 2*pix_dim+18,];
    let monster_offsets5:[usize;MONSTER_OFFSET_COUNT] = [1, pix_dim+1, pix_dim+2, 2*pix_dim+1, 3*pix_dim,
        6*pix_dim, 7*pix_dim+1, 8*pix_dim+1, 9*pix_dim,
        12*pix_dim, 13*pix_dim+1, 14*pix_dim+1, 15*pix_dim,
        18*pix_dim, 19*pix_dim+1];
    let monster_offsets6:[usize;MONSTER_OFFSET_COUNT] = [1, 4, 7, 10, 13, 16,
        pix_dim, pix_dim+5, pix_dim+6, pix_dim+11, pix_dim+12, pix_dim+17, pix_dim+18, pix_dim+19,
        2*pix_dim+18,];
    let monster_offsets7:[usize;MONSTER_OFFSET_COUNT] = [1, pix_dim+2,
        4*pix_dim+2, 5*pix_dim+1, 6*pix_dim+1, 7*pix_dim+2,
        10*pix_dim+2, 11*pix_dim+1, 12*pix_dim+1, 13*pix_dim+2,
        16*pix_dim+2, 17*pix_dim+1, 18*pix_dim, 18*pix_dim+1, 19*pix_dim+1,];
    let mut monster_count = 0;
    // Check for horizontal monsters
    for py in 0..pix_dim-MONSTER_HEIGHT {
        for px in 0..pix_dim-MONSTER_WIDTH {
            let pi = py*pix_dim+px;
            {
                let mut found = true;
                for offset in monster_offsets0.iter() {
                    if image[pi+offset] != b'#' {
                        found = false;
                        break;
                    }
                }
                if found {
                    monster_count += 1;
                    println!("Found monster at [{},{}] orientation 0", px,py);
                }
            }
            {
                let mut found = true;
                for offset in monster_offsets2.iter() {
                    if image[pi+offset] != b'#' {
                        found = false;
                        break;
                    }
                }
                if found {
                    monster_count += 1;
                    println!("Found monster at [{},{}] orientation 2", px,py);
                }
            }
            {
                let mut found = true;
                for offset in monster_offsets4.iter() {
                    if image[pi+offset] != b'#' {
                        found = false;
                        break;
                    }
                }
                if found {
                    monster_count += 1;
                    println!("Found monster at [{},{}] orientation 4", px,py);
                }
            }
            {
                let mut found = true;
                for offset in monster_offsets6.iter() {
                    if image[pi+offset] != b'#' {
                        found = false;
                        break;
                    }
                }
                if found {
                    monster_count += 1;
                    println!("Found monster at [{},{}] orientation 6", px,py);
                }
            }
        }
    }
    // Check for vertical monsters if we haven't found any yet
    for py in 0..pix_dim-MONSTER_WIDTH {
        for px in 0..pix_dim-MONSTER_HEIGHT {
            let pi = py*pix_dim+px;
            {
                let mut found = true;
                for offset in monster_offsets1.iter() {
                    if image[pi+offset] != b'#' {
                        found = false;
                        break;
                    }
                }
                if found {
                    monster_count += 1;
                    println!("Found monster at [{},{}] orientation 1", px,py);
                }
            }
            {
                let mut found = true;
                for offset in monster_offsets3.iter() {
                    if image[pi+offset] != b'#' {
                        found = false;
                        break;
                    }
                }
                if found {
                    monster_count += 1;
                    println!("Found monster at [{},{}] orientation 3", px,py);
                }
            }
            {
                let mut found = true;
                for offset in monster_offsets5.iter() {
                    if image[pi+offset] != b'#' {
                        found = false;
                        break;
                    }
                }
                if found {
                    monster_count += 1;
                    println!("Found monster at [{},{}] orientation 5", px,py);
                }
            }
            {
                let mut found = true;
                for offset in monster_offsets7.iter() {
                    if image[pi+offset] != b'#' {
                        found = false;
                        break;
                    }
                }
                if found {
                    monster_count += 1;
                    println!("Found monster at [{},{}] orientation 7", px,py);
                }
            }
        }
    }
    assert_ne!(0, monster_count, "Didn't find any monsters :(");

    // Count non-sea-monster roughness
    let hash_count = image.iter().fold(0, |count,p| if *p == b'#' {count + 1} else {count});
    (hash_count - MONSTER_OFFSET_COUNT*monster_count).to_string()
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

const _IMAGE_OUTPUT1: &str = "\
.#.#..#.##...#.##..#####\
###....#.#....#..#......\
##.##.###.#.#..######...\
###.#####...#.#####.#..#\
##.#....#.##.####...#.##\
...########.#....#####.#\
....#..#...##..#.#.###..\
.####...#..#.....#......\
#..#.##..#..###.#.##....\
#.####..#.####.#.#.###..\
###.#.#...#.######.#..##\
#.####....##..########.#\
##..##.#...#...#.#.#.#..\
...#..#..#.#.##..###.###\
.#.#....#.##.#...###.##.\
###.#...#..#.##.######..\
.#.#.###.##.##.#..#.##..\
.####.###.#...###.#..#.#\
..#.#..#..#.#.#.####.###\
#..####...#.#.#.###.###.\
#####..#####...###....##\
#.##..#..#...#..####...#\
.#.###..##..##..####.##.\
...###...##...#...#..###";

const _ROT_INPUT: &str = "\
.....\
.#.#.\
...#.\
#....";
const _ROT_OUTPUT90: &str = "\
#...\
..#.\
....\
.##.\
....";
const _ROT_OUTPUT180: &str = "\
....#\
.#...\
.#.#.\
.....";
const _ROT_OUTPUT270: &str = "\
....\
.##.\
....\
.#..\
...#";

#[test]
fn test_day20_rotr_image() {
    let rot90 = rotr90_image(_ROT_INPUT.as_bytes(), 5, 4);
    let rot180 = rotr90_image(&rot90, 4, 5);
    let rot270 = rotr90_image(&rot180, 5, 4);
    let rot360 = rotr90_image(&rot270, 4, 5);
    assert_eq!(_ROT_OUTPUT90.as_bytes(), rot90);
    assert_eq!(_ROT_OUTPUT180.as_bytes(), rot180);
    assert_eq!(_ROT_OUTPUT270.as_bytes(), rot270);
    assert_eq!(_ROT_INPUT.as_bytes(), rot360);
}

const _FLIPH_INPUT: &str = "\
#.##.\
.##.#\
#..#.\
.#..#";
const _FLIPH_OUTPUT: &str = "\
.##.#\
#.##.\
.#..#\
#..#.";

#[test]
fn test_day20_fliph_image() {
    let flip1 = fliph_image(_FLIPH_INPUT.as_bytes(), 5, 4);
    let flip2 = fliph_image(&flip1, 5, 4);
    assert_eq!(_FLIPH_OUTPUT.as_bytes(), flip1);
    assert_eq!(_FLIPH_INPUT.as_bytes(), flip2);
}

#[test]
fn test_day20_assemble_image() {
    let input = parse_input_text(_TEST_INPUT1);
    let tile_dim = input.dim;
    let pix_dim = tile_dim * (10 - 2);
    let image = assemble_image(&input, tile_dim, pix_dim);
    assert_eq!(_IMAGE_OUTPUT1, String::from_utf8(image).unwrap());
}

#[test]
fn test_day20_part2() {
    process_text(_TEST_INPUT1, solve_part2, "273");
}

fn main() {
    println!(
        "Part 1: {}",
        process_file("inputs/input20.txt", solve_part1, "15670959891893")
    );
    println!(
        "Part 2: {}",
        process_file("inputs/input20.txt", solve_part2, "2519") // too high
    );
}
