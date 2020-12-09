My [Advent of Code 2020](https://adventofcode.com/2020) solutions, implemented in
[Rust](https://www.rust-lang.org/) and built using [VS Code](https://code.visualstudio.com/).

## TIL

A list of the puzzles, and what new language/tool features I learned each day:

### [Day 1: Report Repair](https://adventofcode.com/2020/day/1)
- Basic Rust + VSCode integration
- Basic Rust Git configuration (`.gitignore`, `.gitattributes`)
- Reading text files line-by-line with `BufReader`
- Parsing strings as integers with `my_str.parse()`
- `Vec` for growable lists

### [Day 2: Password Philosophy](https://adventofcode.com/2020/day/2)
- All-at-once text file parsing using `fs::read_to_string()`. Nothing this month should be large enough to warrant `BufReader`.
- `String::lines()` to iterate over lines of text data.
- Unit tests with `#[test]`
- Raw strings with `r"blah"`
- Indexing into a string's `char`s is verbose. For AoC purposes, we'll assume ASCII and 1 `u8` = 1 character.
- Basic use of the [`regex`](https://docs.rs/regex) crate -- checking for matches, named capture groups, etc.
- How to set up Cargo for multi-target packages. Less duplication of target metadata this way.
- **Q:** Why is VS Code inserting suggestions after `.` or `:` in comments?
- **Q:** How to generalize this code into a template more easily applicable to future days' problems?

### [Day 3: Toboggan Trajectory](https://adventofcode.com/2020/day/3)
- Basic 2D grid struct with new/get/set methods
- `vec!` macro for array-like `Vec` declarations
- `${fileBasenameNoExtension}` substitution works in `launch.json` (no more day-specific debug configurations!)

### [Day 4: Passport Processing](https://adventofcode.com/2020/day/4)
- Debugging Rust unit tests in VS Code (use the target/debug/deps/project-hash.exe executable, run with `--test-threads=1` to keep the callstack from jumping around)
- Basic HashMap usage
- Custom message for assert failures
- match statement, with non-fatal `None` handling for `Option<T>`

### [Day 5: Binary Boarding](https://adventofcode.com/2020/day/5)
- radix conversion with `u32::from_str_radix()`
- in-place array sorting: `a.sort()`
- `std::cmp::max(a,b)` and `std::cmp::min(a,b)`

### [Day 6: Custom Customs](https://adventofcode.com/2020/day/6)
- nothing new, really