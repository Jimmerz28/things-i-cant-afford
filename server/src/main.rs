#![feature(proc_macro_hygiene, decl_macro)]

// Declare the external dependencies
extern crate juniper;
extern crate juniper_rocket;

// Which macro we're using?
#[macro_use]
extern crate rocket;

// Creates a local name binding but does _not_ link
// Basically just a convenience alias
use rocket::response::content;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![graphiql]
        )
        .launch();
}
