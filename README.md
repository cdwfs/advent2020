My [Advent of Code 2020](https://adventofcode.com/2020) solutions, implemented in
[Rust](https://www.rust-lang.org/) and built using [VS Code](https://code.visualstudio.com/).

## TIL

### [Day 1: Report Repair](https://adventofcode.com/2020/day/1)
- Basic Rust + VSCode integration
- Basic Rust Git configuration (`.gitignore`, `.gitattributes`)
- Reading text files line-by-line with `BufReader`
- Parsing strings as integers with `my_str.parse()`
- `Vec` for growable lists
- *TODO*: Unit tests

### [Day 2: Password Philosophy](https://adventofcode.com/2020/day/2)
- All-at-once text file parsing using `fs::read_to_string()`. Nothing this month should be large enough to warrant `BufReader`.
- `String::lines()` to iterate over lines of text data.
- Unit tests with `#[test]`
- Raw strings with `r"blah"`
- Indexing into a string's `char`s is verbose. For AoC purposes, we'll assume ASCII and 1 `u8` = 1 character.
- Basic use of the [`Regex`](https://docs.rs/regex) crate -- checking for matches, named capture groups, etc.
- How to set up Cargo for multi-target packages. Less duplication of target metadata this way.
- **Q:** Why is VS Code inserting suggestions after `.` or `:` in comments?
- **Q:** How to generalize this code into a template more easily applicable to future days' problems?