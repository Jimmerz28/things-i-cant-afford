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

// Seems there's several syntax varieties
// https://github.com/graphql-rust/juniper/blob/master/juniper_rocket/examples/rocket_server.rs
// https://github.com/ELD/rocket-juniper-example/blob/master/src/graphql/schema.rs
// Going with the macro variant as they're basically decorators
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
