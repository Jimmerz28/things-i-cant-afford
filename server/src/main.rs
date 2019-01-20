#![feature(proc_macro_hygiene, decl_macro)]

// Declare the external dependencies
extern crate juniper;
extern crate juniper_rocket;

// Which macro we're using?
#[macro_use]
extern crate rocket;

// Imports everything from ./routes.rs
// @TODO: Figure out how to namespace local files' functions
mod routes;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![routes::graphiql]
        )
        .launch();
}
