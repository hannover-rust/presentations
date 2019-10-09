mod types;
mod other;

fn main() {
    use types::foo;
    types::bar();
    foo();
    other::other();
}
