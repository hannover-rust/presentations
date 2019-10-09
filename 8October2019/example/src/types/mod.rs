
mod foo;

pub use self::foo::foo;

pub fn bar() {
    println!("bar")
}
