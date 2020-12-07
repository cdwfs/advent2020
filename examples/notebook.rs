use rand::Rng;
use regex::Regex;
use std::cmp::Ordering;
use std::io;

// Function parameters MUST have an explicit type, thankfully
// But it can be generic and defined by a set of traits instead of a concrete type.
// If a function has a return value, it must also be explicit. The absence of a return
// type means the function is void.
fn _toy_function(x: i32) -> i32 {
    x
}

// Tuple struct example, used below
struct Point3D(f32, f32, f32);

// Struct example, used below
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    birth_year: i32,
}

impl User {
    fn name_length(&self) -> usize {
        self.name.len()
    }

    // self can be a mutable reference if the method should modify it
    fn set_birth_year(&mut self, new_birth_year: i32) {
        self.birth_year = new_birth_year;
    }
}

// Multiple impl blocks for the same struct are fine
impl User {
    // Methods without a "self" parameter are "associated methods", Rust-ese for C++ static functions
    // This is commonly used for new()/create() methods
    fn _new(name: String, email: String, birth_year: i32) -> User {
        User {
            name,
            email,
            birth_year,
        }
    }
}

fn main() {
    // TUPLES
    let t: (i32, &str, f32) = (7, "beans", 4.0);
    // extract individual tuple values
    let (i, _, f) = t;
    assert_eq!(7, i);
    assert_eq!(4.0, f);
    // or access them directly by index
    assert_eq!(7, t.0);
    assert_eq!("beans", t.1);
    // Tuple structs are nice in cases where a full struct would be overkill, I guess?
    let pt = Point3D(1.0, 2.0, 3.0);
    assert_eq!(pt.1, 2.0);

    // STRUCTS
    // initialize structs by providing every field
    let person1 = User {
        email: String::from("me@aol.com"),
        name: String::from("Joe Me"),
        birth_year: 1975,
    };
    // named field access works as expected
    assert_eq!(1975, person1.birth_year);
    // So do method calls
    assert_eq!(6, person1.name_length());
    // Entire instance must be mutable; it can't be just individual fields
    let mut person2 = User {
        email: String::from("him@aol.com"),
        name: String::from("Jill Him"),
        birth_year: 1979,
    };
    person2.email = String::from("jhim@aol.com");
    // Methods with a mutable self reference require a mutable instance
    // person1.set_birth_year(1964); // ERROR: person1 is not mutable
    person2.set_birth_year(1964);
    assert_eq!(1964, person2.birth_year);
    // Rust automatically figures out whether you're calling a method on a struct, a reference, or
    // a mutable reference.
    (&mut person2).set_birth_year(787);
    assert_eq!(787, person2.birth_year);
    // If a variable exists in scope with the same name as a struct field, you can
    // use it directly instead of specifying both field and value
    let email = String::from("jhim@aol.com");
    let person2 = User {
        email, // shorthand for "email: email"
        name: person2.name,
        birth_year: person2.birth_year,
    };
    // Creating an instance from an existing instance while customizing specific fields:
    let person2_alias = User {
        email: String::from("jhim.work@business.com"),
        name: String::from("Jill Him (work)"),
        ..person2 // copy remaining fields from person2 (no trailing comma allowed; must be last)
    };
    assert_eq!(person2.birth_year, person2_alias.birth_year);
    //println!("{}", person2); // ERROR: Displaying custom types doesn't work by default, needs the Display trait.
    println!("{:?}", person2); // Works, but only if the User struct derives the std::fmt::Debug trait!
    println!("{:#?}", person2); // Also works, but prints one field per line (good for larger structs)

    // ARRAYS
    // array is fixed-size, like std::array
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.len(), 5);
    // println!("{}", a); // ERROR: won't compile, can't format arrays by default
    println!("{:?}", a); // ...but this works
                         // The type of an array includes its length.
                         // let b:[i32] = [1,2,3]; // ERROR: won't compile, missing length. [i32] is a slice.
                         // let b:[i32;4] = [1,2,3]; // ERROR: won't compile, value has different length than variable.
    let _b: [i32; 3] = [1, 2, 3];
    // Shortcut to create an array with many copies of the same value:
    let fives = [5; 15];
    assert_eq!(15, fives.len());
    // out-of-bounds accesses don't work.
    // let a5 = a[5]; // ERROR: will panic at runtime (and may even fail to compile!)

    // VECTORS
    // Vec is std::vector

    // IF
    // "if" is an expression in Rust
    let x = 5;
    let y = if x == 5 { 7 } else { 9 };
    assert_eq!(7, y);

    // LOOP
    // "loop" is also an expression; "break" takes an optional value that becomes the value of the
    // loop expression.
    let mut loop_accumulator = 1;
    let loop_threshold = 1000;
    let loop_value = loop {
        loop_accumulator *= 2;
        if loop_accumulator > loop_threshold {
            break loop_accumulator;
        }
    };
    assert_eq!(1024, loop_value);

    // FOR
    // for loops are iterator-based, but you must explicitly retrieve an iterator from a collection
    let for_elems = [0, 2, 4, 6, 8];
    for elem in for_elems.iter() {
        assert_eq!(0, elem % 2);
    }
    // Iterators can be enumerated, to get both the zero-based index and the value of each element
    for (i, &elem) in for_elems.iter().enumerate() {
        assert_eq!(elem, 2 * i);
    }
    // There is a standard Range type for iterating over a range of integer values
    let mut sum = 0;
    for i in 1..5 {
        // Range is inclusive at the low end, exclusive at the top; i will be 1, 2, 3, 4
        sum += i;
    }
    assert_eq!(10, sum);
    // Ranges can also be reversed
    let mut sum = 0;
    for i in (1..5).rev() {
        // same range, but now i will be 4, 3, 2, 1
        sum += i;
    }
    assert_eq!(10, sum);

    // SLICES
    let s = String::from("beans and franks");
    // Slice indices are *BYTE* indices into a utf8 string, not *CHARACTER* indices. Indexing into the middle
    // of a multi-byte character is an error.
    let second_word: &str = &s[6..9];
    assert_eq!("and", second_word);
    assert_eq!("beans", &s[..5]); // If the beginning of a slice isn't specified, it's implicitly 0
    assert_eq!("franks", &s[10..]); // If the end of a slice    isn't specified, it's implicitly "end of string"
    assert_eq!("beans and franks", &s[..]); // both ends can be omitted, and you get a slice for the whole string
                                            // String literals are already slices; they have type &str
    let s_lit = "bangers and mash";
    assert_eq!(s_lit, &s_lit[..]);
    // Slices work for arrays as well, not just strings.

    // REFERENCES
    let s_original = String::from("beans");
    let s = s_original; // ownership transfer; s_original is now uninitialized
    let _s_ref = &s; // immutable reference; this is fine.
    let _s_ref2 = &s; // multiple immutable references, still fine
                      // let s_mut_ref = &mut s; // ERROR -- can't take a mutable reference to a non-mutable variable!
    let mut s = s; // transfer ownership and make mutable; this is fine.
    let s_mut_ref = &mut s; // mutable reference to mutable variable -- fine
    s_mut_ref.make_ascii_uppercase();
    assert_eq!("BEANS", &s_mut_ref[..]);
    assert_eq!("BEANS", &s[..]); // the original s is still fine (taking a reference doesn't transfer ownership), but...
                                 // assert_eq!("BEANS", &s_mut_ref[..]); // ERROR: ...doing so invalidates references.

    // REGEX
    let line = "1-3 a: abacd";
    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<c>[a-z]): (?P<pw>[a-z]+)$").unwrap();
    assert!(re.is_match(line));
    let caps = re.captures(line).unwrap();
    assert_eq!("abacd", caps.name("pw").unwrap().as_str());
}
fn _guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is {}", secret_number);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
