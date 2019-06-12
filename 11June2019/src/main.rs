extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
        .mount("/examples", StaticFiles::from("examples/"))
        .mount("/slides", StaticFiles::from("slides/"))
        .launch();
}
