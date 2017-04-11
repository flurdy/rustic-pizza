#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::Template;


#[get("/pizza")]
fn show_menu() -> Template {
   let mut context = HashMap::new();
   context.insert("pizzas",vec!["Margherita", "Pepperoni", "Hawaii"]);
   Template::render("pizza_menu", &context)
}

fn main() {
   rocket::ignite().mount("/", routes![show_menu]).launch();
}

#[cfg(test)]
mod tests {
   use super::rocket;
   use rocket::testing::MockRequest;
   use rocket::http::{Status, Method};

   #[test]
   fn pizza_menu_test() {
      let rocket = rocket::ignite().mount("/", routes![super::show_menu]);
      let mut req = MockRequest::new(Method::Get, "/pizza");
      let mut response = req.dispatch_with(&rocket);
      assert_eq!(response.status(), Status::Ok);
      let body = response.body()
                    .and_then(|b| b.into_string())
                    .unwrap_or("No body".to_string());
      assert!(body.contains("Pepperoni"));
   }
}
