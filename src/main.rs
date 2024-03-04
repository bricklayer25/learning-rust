use std::any::Any;
// the above line is a use statement, which is used to bring a module into scope (the modules are located in files called crates)
// Q: what does std::any::Any do?
// A: it is a trait that is implemented by all types, and can be used to downcast to a concrete type
// Q: what is downcasting?
// A: downcasting is the process of converting a reference of a base type to a reference of a derived type

// https://www.youtube.com/watch?v=br3GIIQeefY&t=304s   -
//  ^^ brief rust crash course and my source for this experimentation code

fn main() {
    println!("Hello, world!");
    let x = {
        let x = 5;
        x + 6
    };
    // basic code block defining what x evaluates to (note that in the final line of the code block, there is no semicolon.)
    // in functions, variable definitions, etc., the last line of the block that doesn't have a semicolon is what the block evaluates to
    println!("The value of x is: {}", x);

    fn print_number(x: i32) {
        match x {
            1 => println!("One"),
            2 => println!("Two"),
            _ => println!("{}", x),
        }
    }

    print_number(1);
    print_number(2);
    print_number(14);

    struct Number{
        odd: bool,
        value: i32,
    }
    impl Number {
        fn is_positive(&self) -> bool {
            self.value > 0
        }
    }
    // function that is a method of the Number struct (self is the instance of the struct)

    // the mut keyword below makes the variable 'z' mutable (rust variables are immutable by default)
    let mut z = Number {odd: true, value: 5};
    println!("Is z positive? {}", z.is_positive());

    z.value = -1;
    println!("Is z positive? {}", z.is_positive());

    // generic types
    struct Pair<T> {
        x: T,    // x and y are of type T which is a generic type (can be any type)
        y: T
    }

    let p = Pair {x: 5, y: 10};
    let okay = Pair {x: "hello", y: "world"};
    println!("p.x = {}", p.x);
    println!("okay.x = {} and ", okay.x);

    // vectors type
    let mut v = Vec::new(); // heap-allocated array that is generic (can hold any type, but not various types)
    v.push(5); // this will make the Vector of type i32
    // v.push(false); this will result in an error because the vector is of type i32
    let mut v2 = Vec::new();
    v2.push("hello"); // this will make v2 of type &str


    // Q: what is the difference between &str and String?
    // A: &str is a string slice, a reference to a string, and String is a heap-allocated string
    // q: so String is effectively a vector of characters?
    // A: yes, it is a vector of characters, but it is a heap-allocated vector of characters
    // Q: well vectors are heap-allocated implicitly, no?
    // A: yes, but the difference is that String is a vector of characters that is specifically a string, while Vec is a vector of any type
    // Q: so in which cases would you use either &str or String?
    // A: &str is used when you want to pass a string to a function, and String is used when you want to create a string
    // Q: so you can't create a string with &str?

    let s: String = "hello".to_string(); // this is how you create a string from a string slice (of type String)
    let s2 = "hello"; // whereas this is a string slice (of type &str)

    let boolean = {
        5
    };
    println!("the above structure is of type i32: {}", boolean.type_id() == 5.type_id());

    let v3 = vec![1, 2, 3];  // taking a look at the vec macro
    // the exclamation mark "!" indicates that the function preceding the exclamation mark is a macro
    // println!("{}", "Hello world!") is a macro too, and expands to something like the following:
    /*
    use std::io::{self, Write};
    io::stdout()
    .lock()
    .write_all(b"Hello, world!\n")
    .unwrap();
     */

    // panic!("down with the ship!"); // this will cause the program to panic (completely halt execution)
    // and print the message "down with the ship!"
    // thread 'main' panicked at src\main.rs:97:5: down with the ship!

    // some methods also panic, such as unwrap which is a method of the Option type (Option types either contain a value (Some) or None)
    // unwrap will panic if the Option type is None (i.e. it contains no value)
    // e.g.

    let o1 : Option<i32> = Some(5);
    let o2: Option<i32> = None;
    o1.unwrap();
    // o2.unwrap(); this will panic

    // Option isn't a struct, but rather an enum (a type that can be one of a few different types)
    // in the case of Option, it can be either Some or None, where Some is generic and can be any type
    // e.g. Option<i32> is either Some(5) or None
    // Option<&str> is either Some("hello") or None
    // Option's implementation looks something like this:
    /*
    enum Option<T> {
        Some(T),
        None,
    }
     */

    // Result is another enum that is similar to Option, but is used for error handling
    // Result is either Ok or Err, where Ok is the result of a successful operation, and Err is the result of an unsuccessful operation, or an error
    // e.g. Result<i32, &str> is either Ok(5) or Err("error message")
    // Result's implementation looks something like this:
    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
     }
     */
    // result behaves similarly to option in that if it is Err, it will panic upon being unwrapped

    // q: what does {:?} do in println!?
    // a: it is a format specifier that is used to print a debug representation of a value
    // e.g. println!("{:?}", "hello") will print "hello"
    // e.g. println!("{:?}", 5) will print 5
    // e.g. println!("{:?}", Some(5)) will print Some(5)
    // e.g. println!("{:?}", None) will print None

    let s1 = std::str::from_utf8(
        &[240, 159, 141, 137]
    );
    println!("{:?}", s1); // this will print Ok("🍉") because the bytes [240, 159, 141, 137] are the UTF-8 representation of the watermelon emoji
    // println!("{}", s1); // this will result in an error because s1 is a string from bytes and needs to be formatted as such
    let s2 = std::str::from_utf8(&[131, 124, 53]);
    // println!("{:?}", s2); // this will print Err(Utf8Error { valid_up_to: 0, error_len: Some(1) })
    // because the bytes [131, 124, 53] as these bytes don't represent a valid UTF-8 string

    // in the case above, s1 represents the OK variant of the Result enum and s2 represents the Err variant of the Result enum

    // we can also use expect to telegraph to the user what we were expecting where a panic could occur
    // s2.expect("valid utf-8");
    // thread 'main' panicked at src\main.rs:155:8:
    // valid utf-8: Utf8Error { valid_up_to: 0, error_len: Some(1) }
    s1.expect("valid utf-8"); // this will not panic because s1 is valid utf-8

/*
    let melon2 = &[240, 159, 141, 137];
    match std::str::from_utf8(melon2) {
        Ok(s) => println!("{}", s),
        Err(e) => return Err(e),
    }
    Ok(());
    */
    // the ? operator is used to return an error if the result is an error, which is logically equivalent to the match statement above
    // e.g. let s = str::from_utf8(melon2)?;

    // iterators:
    // common iterator notation is as such:

    let i = 1..; // this is the iterator for natural numbers (1, 2, 3, 4, 5, ...)
    // storing this in memory is feasible thanks to rust lazy evaluation (i.e. the iterator is not evaluated until it is needed)
    // this is a range, which is a type of iterator that is used to generate a sequence of numbers
    // the range is inclusive of the first number and exclusive of the second number by default
    (0..).contains(&100); // true
    // numbers from 0 to 20 (not including 20)
    (..20).contains(&20); // false
    // only 3, 4, 5 (not 6)
    (3..6).contains(&4); // true
    // only 3, 4, 5, 6
    (3..=6).contains(&6); // true

    // anything that is iterable can be used in a for loop
    for i in 0..=5 {
        println!("{}", i);
    }

    for i in vec![42, 45, 12] {
        println!("{}", i);
    }

    for c in "rust".chars() {
        println!("{}", c);
    }


}
