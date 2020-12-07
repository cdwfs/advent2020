use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("inputs/input01.txt") {
        let mut numbers = Vec::new();
        for line in lines {
            if let Ok(num) = line {
                let my_int = num.parse::<i32>().unwrap();
                numbers.push(my_int);
            }
        }
        for a in 0..numbers.len() {
            let na = numbers[a];
            for b in a + 1..numbers.len() {
                let nb = numbers[b];
                for c in b + 1..numbers.len() {
                    let nc = numbers[c];
                    if na + nb + nc == 2020 {
                        println!(
                            "{} + {} + {} = 2020, answer is {}",
                            na,
                            nb,
                            nc,
                            na * nb * nc
                        );
                    }
                }
            }
        }
    }
}
