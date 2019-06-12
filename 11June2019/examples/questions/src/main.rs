
use std::io::prelude::*;
use std::fs::File;

fn ex1() {
    let mut file = match File::open("test.txt") {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            panic!("Couldn't Read file")
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for (idx, line) in contents.lines().enumerate() {
        let needle = "important";
        if line.contains(needle) {
            println!("Found {} on line {}", needle, idx + 1);
        }
    }

    // println!("{}", contents);
}

fn ex2() {
    let mut arr = [1, 2, 3, 4];

    arr.iter_mut()
        .map(|x| *x += 1 );

    println!("{:?}", arr);

    let box_array = Box::new([1, 2, 3, 4]);
}

fn main() {
    ex1();
    ex2();
}
