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

### [Day 11: Seating System](https://adventofcode.com/2020/day/11)
- `std::mem::swap()` to exchange two mutable locations without deinitializing
- No ternary operator; `if-else` expressions have a value instead: `x = if (c) { a } else { b };
- integer casts: `usize::try_from(n).unwrap()`
- `#[rustfmt::skip]` on a function/etc. to opt out of `rustfmt`

### [Day 12: Rain Risk](https://adventofcode.com/2020/day/12)
- absolute value is a method on integer types, e.g. `-3.abs()`
- No Python-esque tuple assignments; if `wx` and `wy` exist, you can't swap with `wx,wy = wy,wx;`

### [Day 13: Shuttle Search](https://adventofcode.com/2020/day/13)
- Probably rediscovered some modular arithmetic theorem? ([Sure enough!](https://en.wikipedia.org/wiki/Chinese_remainder_theorem))

### [Day 14: Docking Data](https://adventofcode.com/2020/day14)
- Rust calls reduce() [`fold()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold).
- Successful use of `HashMap::entry()`: `*memory.entry(addr).or_default() = val;`

### [Day 15: Rambunctious Recitation](https://adventofcode.com/2020/day15)
- HashMaps are _slow_ in debug builds.
- Need to come back and reoptimize/clean this up so I can re-enable the unit tests for part 2, I'm doing a gajillion unnecessary lookups.

### [Day 16: Ticket Translation](https://adventofcode.com/2020/day16)
- `cargo clippy`! Went back and fixed [all the clippy lints](https://rust-lang.github.io/rust-clippy/master/index.html) in previous days' solutions.
- Use 64-bit integers everywhere!

### [Day 17: Conway Cubes](https://adventofcode.com/2020/day17)
- HashSet
- static constant arrays
- Tuple structs
- Initializing collections with `.with_capacity()` instead of `new()` if an upper-bound (or exact) size is known at creation time.

### [Day 18: Operation Order](https://adventofcode.com/2020/day18)
- Some new string iteration functions: `.matches()`, `.match_indices()`, `.position()`, etc. But, definitely not clear which are defined on which string types.
- I am not good at `String`/`str`/`&str`/`[u8]` manipulation in Rust :(. In part 2 especially, I'm creating new `String`s all over the place just to keep the borrow checker happy.

### [Day 19: Monster Messages](https://adventofcode.com/2020/day19)
- Iterator::any() and all()
- Escape curly braces in format strings with `{{` and `}}`

### [Day 20: Jurassic Jigsaw](https://adventofcode.com/2020/day/20)
