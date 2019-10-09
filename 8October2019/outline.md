# Rust meetup 8. October 2019

## Theme: Getting rid of `clone()` and other "Design Patterns"

The rules of the borrow checker:
* There is only one owner of a resource
* One or more references (&T) to a resource
* Exactly one mutable (&mut T) to a resource

Excessive use of `clone()` in rust is usually considered a "code smell".

The borrow-checker in rust makes sure that all data has a single owner.

This can become a problem for specific types of problems such as graphs, where
the parent and child objects need to have references to each-other.

To "defeat the borrow-checker" it's pretty common to just `clone()` the data
which can't be directly borrowed.

The best way is to restructure your program such that you have a linear
data-flow, but sometimes this isn't possible. 

The 'best' solution is very problem specific, but Rust has a few built-in types
to help, in this case.
 
Using a reference-counted type(s) such as Rc<T> or Arc<T> can help.
```rust
use std::rc::Rc;
use std::rc::Weak;
fn main() {
    let foo = Rc::new(String::from("foo"));
    let bar: Rc<String>   = foo.clone();
    let baz: Weak<String> = foo.downgrade();
}
```

Even then you need to sort out 'strong' and 'weak' references to an object.

* 'strong' references contribute to the reference count
* 'weak' references don't (and need to be checked before each use if it's still valid)

An example would be in a tree structure to have parents have strong references
to their children and the children weak references to their parents. Doing this
helps prevents cycles.

A cycle in an Rc<T> means that the object will never be dropped causing a
memory-leak. (This is still considered 'memory-safe'!)

The module level docs for `std::cell` have a good explanation.
https://doc.rust-lang.org/std/cell/index.html

### Interior Mutability: Cell<T>, RefCell<T>

```rust
fn main() {
    let foo: Rc<Cell<T>> = Rc::new(Cell::new(String::new()));
    let bar = foo.clone();
    foo.borrow_mut();
    bar.borrow();
}
```

## Other Rust "Design Patterns" (that might help with "lifetime-management")

### mem::replace() instead of cloning intermediate values

Unneeded clone():
```rust
fn main() {
    let mut v: Vec<i32> = vec![1, 2];
    let old_v = v.clone();
    v = vec![3, 4, 5];
}
```

With `mem::replace()`
```rust
use std::mem;
fn main() {
    let mut v: Vec<i32> = vec![1, 2];
    let old_v = mem::replace(&mut v, vec![3, 4, 5]);
}
```

The docs have another important use of mem::replace(), to get out of the
'error: cannot move out of dereference of `&mut`-pointer' problems.

### Struct composition

creating smaller units of data which can be shared separately.

### Builder pattern

Using a new 'Builder' type to set properties of static object which may need to
be checked for validity before instantiation/use.
```rust

struct WindowBuilder { window: Window }
struct Window {}

impl WindowBuilder {
    fn new() -> WindowBuilder { 
        WindowBuilder { window: Window {} }
    }

    fn with_width(&mut self, size: i32) -> &mut Self {}
    
    fn build(self) -> Window {
        self.window
    }
}

fn main() {
    let obj: Window = WindowBuilder::new()
        .with_width(100)
        .build();
}
```

### New-type Pattern

wrapping a type to control the interface to the underlying type and to add some
more control of types

### `Unsafe {}` stops at module boundaries

The use of `unsafe {}` does not necessarily stop outside the scope of the
unsafe block.

```rust
fn main () {
    unsafe {
    }
}
```
 
The best way to limit the side-effects of unsafe is to contain it with a module.

### Interfaces over platform specific code

You can define a generic interface and only compile one version.

An example of this is the `libc` rust library.

```rust
#[cfg(windows)]
mod windows {
    pub fn print() -> &'static str { "windows" }
}

#[cfg(unix)]
mod unix {
    pub fn print() -> &'static str { "unix" }
}

#[cfg(windows)]
use crate::windows::print;
#[cfg(unix)]
use crate::unix::print;
    
fn main() {
    println!("{}", print())
}
```

