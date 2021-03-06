#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;

#[cfg(test)] mod tests;

use std::collections::HashMap;
use std::sync::Mutex;
use rocket::State;
use rocket_contrib::Template;
use rocket::response::{Failure, Redirect};
use rocket::http::Status;
use rocket::request::Form;
use uuid::Uuid;


static PIZZAS: &'static [&'static str] = &["Margherita", "Pepperoni", "Hawaii"];

#[get("/pizza")]
fn show_menu() -> Template {
   let mut context = HashMap::new();
   context.insert("pizzas",PIZZAS);
   Template::render("pizza_menu", &context)
}

#[get("/pizza/order/<order_id>")]
fn show_pizza_ordered(order_id: String, database: State<PizzaOrderDatabase>) -> Result<Template, Failure> {
   match Uuid::parse_str(order_id.as_str()) {
      Ok(order_id) => {
         match database.lock().unwrap().get(&order_id) {
            Some(..) => {
               let mut context = HashMap::new();
               context.insert("order_id", order_id);
               Ok(Template::render("pizza_ordered", &context))
            },
            None => {
               println!("Pizza order id not found: {}", &order_id);
               Err(Failure(Status::NotFound))
            }
         }
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

type PizzaOrderDatabase = Mutex<HashMap<Uuid, String>>;

#[post("/pizza/order", data = "<pizza_order_form>")]
fn order_pizza(pizza_order_form: Form<PizzaOrder>, database: State<PizzaOrderDatabase>) -> Result<Redirect, Failure> {
   let pizza_order = pizza_order_form.get();
   let pizza_name = &pizza_order.name;
   let pizzas: Vec<String> = PIZZAS.iter().map(|p| p.to_string().to_lowercase()).collect();
   if pizzas.contains(&pizza_name.to_lowercase()){
      println!("Pizza ordered: {}", &pizza_name);
      let order_id = Uuid::new_v4();
      database.lock().unwrap().insert(order_id.clone(), pizza_name.clone().to_lowercase() );
      Ok(Redirect::to(format!("/pizza/order/{}",order_id).as_str()))
   } else {
      println!("Pizza ordered not found: {}", &pizza_name);
      Err(Failure(Status::NotFound))
   }
}

fn mount_rocket() -> rocket::Rocket {
   rocket::ignite()
           .manage(Mutex::new(HashMap::<Uuid,String>::new()))
           .mount("/", routes![show_menu,order_pizza,show_pizza_ordered])
}

fn main() {
   mount_rocket().launch();
}
