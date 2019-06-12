//! Module documentation
//! ```
//! fn foo() -> &'static str {
//!     "Can also display markdown"
//! }
//! ```


#[allow(dead_code, unused)]


/// Function documentation
fn main() {
    // variables & types
    {
        let  x = 5;    // types will be inferred based on usage...
        // let x: i32 = 5; // ... but can be defined at declaration
        //let mut x = 5;   // mutation not allowed unless variable is `mut`able

        x += 1;
    }

    // primitive types
    {
        let bool = true;
        let int = 0;
        let float = 0.0;
        let tuple = (2, "cat", 42.0);
        let array = [0, 1, 2];
        let reference = &tuple;
        let slice = &array[1..]; // reference (and len) to section of array
        let char = 'f';
        let string_reference = "Hello, World!";
    }

    // functions & printing
    {
        // Everything important about a function is declared in the signature!

        // `fn` declares a function
        // `(...)` declares the parameter list
        // `-> ...` is the return type
        fn add_one(x: i32) -> i32 {
            // Rust is an "expression oriented" language which means
            // that we don't need an explicit `return`, the last expression
            // is the returned.
            x + 1
        }
        assert_eq!(add_one(1), 2);
    }

    // printing to `stdout`
    {
        println!("Hello, World!");

        // formatted print with `{}`
        println!("1 + 1 = {}", 2);

        #[derive(Debug)]
        struct Foo { bar: Vec<i32> }
        let foo = Foo { bar: vec!(1, 2, 3) };

        // debug output with `{:?}`
        println!("{:?}", foo);
        // pretty print debug output with `{:#?}`
        println!("{:#?}", foo);
    }

    // `if` statements & more expressions
    {
        let value = 42;

        // must be a boolean statement!
        if value == 42 {
            println!("true");
        } else {
            println!("false");
        }

        // `if` is also an expression!
        let new_value = if value > 50 { 100 } else { 0 };
    }

    // `for`, `while`, and `loop`
    {
        println!("range : 0..5");
        for idx in 0..5 {
            println!("index: {}", idx);
        }

        let mut done = false;
        while !done {
            println!("See you in a while!");
            done = true;
        }

        loop {
            println!("I'm in a loop!");
            break;
        }
    }

    // Enums & pattern matching
    {
        enum State {
            Stopped,
            Running(i32),
            Error(String),
        }

        let state = State::Stopped;

        match state {
            State::Stopped => println!("Stopped"),
            State::Running(x) => println!("Running at {}", x),
            State::Error(err) => println!("Error: {}", err),
        }
    }

    // structs & `impl` methods & Traits
    {
        #[derive(Debug)] // Some traits can be automatically generated
        struct Point {
            x: f32,
            y: f32,
        }

        impl Point {
            fn new(x: f32, y: f32) -> Point {
                Point { x, y }
            }
        }

        let point = Point::new(1.0, 2.0);

        // Traits are implementation contracts (aka interfaces)
        // Composition of common functionality
        // An example is the `Default` trait in the standard library, which is
        // used when a default value is wanted/needed and is defined as
        // ```
        // pub trait Default {
        //     fn default() -> Self;
        // }
        // ```
        // and implemented as follows:

        #[allow(unused_imports)]
        use std::default::Default; // already included in `std::prelude::*`

        impl Default for Point {
            fn default() -> Point {
                Point { x: 0.0, y: 0.0 }
            }
        }

        let default_point = Point::default();

        println!("point: {:?}", point);
        println!("default: {:?}", default_point);

        // Requires impl of `std::format::Display` trait
        // println!("point: {}", point);
    }

    // Option, Result, and error handling
    {
        // Note: `T` and `E` here are just generic type signatures

        // `Option` is used if a value could be `undefined` or doesn't exist
        // It is defined as:
        // ```
        // enum Option<T> {
        //     None,
        //     Some(T),
        // }
        // ```
        let opt_example = Some(42);
        match opt_example {
            None => println!("Got nothing to show."),
            Some(val) => println!("Got value : {}", val)
        }

        // `Result` is used when an error could occur
        // It is defined as:
        // ```
        // #[must_use]
        // enum Result<T, E> {
        //     Ok(T),
        //     Some(E),
        // }
        // ```
        // The `#[must_use]` compiler directive means that the value must be
        // assigned or otherwise used. This is to prevent blindly calling
        // functions which could fail.
        let result: Result<&str, &str> = Ok("Some Text");
        match result {
            Ok(val) => println!("Got: {}", val),
            Err(err) => println!("Error: {}", err)
        }

        // `panic!` is used for unrecoverable errors
        // panic!("Oops...");

        // Warning: Some std-lib functions panic!
        {
            let array = vec![1, 2, 3, 4];
            let val = array[20]; // <- panic!

            // `unwrap()` can directly get a value from an `Option` or `Result`
            // but panics on `None` or `Error` respectively
            let opt: Option<i32> = None;
            let val = opt.unwrap(); // <- panic!
        }
    }
}
