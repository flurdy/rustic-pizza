#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;

#[cfg(test)] mod tests;

use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::response::{Failure, Redirect};
use rocket::http::Status;
use rocket::request::Form;
use uuid::Uuid;


#[get("/pizza")]
fn show_menu() -> Template {
   let mut context = HashMap::new();
   let pizzas = vec!["Margherita", "Pepperoni", "Hawaii"];
   context.insert("pizzas",pizzas);
   Template::render("pizza_menu", &context)
}

#[get("/pizza/order/<order_id>")]
fn show_pizza_ordered(order_id: String) -> Result<Template, Failure> {
   match Uuid::parse_str(order_id.as_str()) {
      Ok(order_id) => {
         let mut context = HashMap::new();
         context.insert("order_id", order_id);
         Ok(Template::render("pizza_ordered", &context))
      },
      Err(..)  => {
         println!("Pizza order id not valid: {}", &order_id);
         Err(Failure(Status::NotFound))
      },
   }
}

#[derive(FromForm)]
struct PizzaOrder {
    name: String,
}

#[post("/pizza/order", data = "<pizza_order_form>")]
fn order_pizza(pizza_order_form: Form<PizzaOrder>) -> Result<Redirect, Failure> {
   let pizza_order = pizza_order_form.get();
   let pizza_name = &pizza_order.name;
   let pizzas: Vec<String> = vec!["Margherita", "Pepperoni", "Hawaii"].iter().map(|p| p.to_string().to_lowercase()).collect();
   if pizzas.contains(&pizza_name.to_lowercase()){
      println!("Pizza ordered: {}", &pizza_name);
      let order_id = Uuid::new_v4();
      Ok(Redirect::to(format!("/pizza/order/{}",order_id).as_str()))
   } else {
      println!("Pizza ordered not found: {}", &pizza_name);
      Err(Failure(Status::NotFound))
   }
}

fn mount_rocket() -> rocket::Rocket {
   rocket::ignite().mount("/", routes![show_menu,order_pizza,show_pizza_ordered])
}

fn main() {
   mount_rocket().launch();
}
