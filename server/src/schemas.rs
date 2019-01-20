#[macro_use]

extern crate juniper;

#[derive(GraphQLObject)]
#[graphql(description="An expensive object I probably can't afford")]
struct ExpensiveItem {
    id: String,
    name: String,
    price: f64,
    url: String
}

#[derive(GraphQLInputObject)]
#[graphql(description="An expensive object I probably can't afford")]
struct NewExpensiveItem {
    name: String,
    price: f64,
    url: String
}
