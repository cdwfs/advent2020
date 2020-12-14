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

### [Day 7: Handy Haversacks](https://adventofcode.com/2020/day/7)
- `HashSet` for value-less hash maps
- `String.split()` returns an iterator, not a collection
- Started to run into lifetime issues. I can solve them by just making things `String` until they go away, but that's not sustainable.

### [Day 8: Handheld Halting](https://adventofcode.com/2020/day/8)
- I made an `enum`. Though in this case it could've been a struct in the end, since NOPs have args as well.
- Q: how to safely add a signed integer to a `usize`? (typecasting around feels like it misses the point)
- I feel like `HashMap.entry()` could save me some redundant lookups, but performance hasn't been an issue yet.

### [Day 9: Encoding Error](https://adventofcode.com/2020/day/9)
- Some basic functional programming with `.map()` and lambdas
- Q: How best to pass side-channel data into the "solve" functions, such as the history size for part 1, or the target sum from part 2? I guess an `enum` of some sort?

### [Day 10: Adapter Array](https://adventofcode.com/2020/day/10)
- Hash map keys _must_ be borrowed. Even if they're literals. (Q: why?)
- `HashMap.Get(&k).unwrap_or(default)` for potentially missing values
