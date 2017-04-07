#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/pizza")]
fn show_menu() -> &'static str {
    "PIZZA Mamma Mia"
}

fn main() {
  rocket::ignite().mount("/", routes![show_menu]).launch();
}
