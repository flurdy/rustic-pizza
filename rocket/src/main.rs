#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;

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

#[derive(FromForm)]
struct PizzaOrder {
    name: String,
}

#[post("/pizza/order", data = "<pizza_order_form>")]
fn order_pizza(pizza_order_form: Form<PizzaOrder>) -> Result<Redirect, Failure> {
    let pizza_order = pizza_order_form.get();
    let pizza_name = &pizza_order.name;
    let pizzas: Vec<String> = vec!["Margherita", "Pepperoni", "Hawaii"].iter().map(|p| p.to_string().to_lowercase()).collect();
    if pizzas.contains(pizza_name){
        println!("Pizza ordered: {}", &pizza_name);
        let order_id = Uuid::new_v4();
        Ok(Redirect::to(format!("/pizza/order/{}",order_id).as_str()))
    } else {
        println!("Pizza ordered not found: {}", &pizza_name);
        Err(Failure(Status::NotFound))
    }
}

fn main() {
   rocket::ignite().mount("/", routes![show_menu,order_pizza]).launch();
}

#[cfg(test)]
mod tests {
   use super::rocket;
   use rocket::testing::MockRequest;
   use rocket::http::{ContentType, Method, Status};
   use uuid::Uuid;

   #[test]
   fn pizza_menu_test() {
      let rocket = rocket::ignite().mount("/", routes![super::show_menu,super::order_pizza]);
      let mut req = MockRequest::new(Method::Get, "/pizza");
      let mut response = req.dispatch_with(&rocket);
      assert_eq!(response.status(), Status::Ok);
      let body = response.body()
                    .and_then(|b| b.into_string())
                    .unwrap_or("No body".to_string());
      assert!(body.contains("Pepperoni"));
   }

   #[test]
   fn pizza_order_test() {
      let rocket = rocket::ignite().mount("/", routes![super::show_menu,super::order_pizza]);
      let mut req = MockRequest::new(Method::Post, "/pizza/order")
                        .header(ContentType::Form)
                        .body(&format!("name={}", "pepperoni"));
      let response = req.dispatch_with(&rocket);
      assert_eq!(response.status(), Status::SeeOther);

      let header = response.headers().find(|h| h.name() == "Location").unwrap();
      let location = header.value();
      let order_id_raw = location.split("/").last().unwrap(); // .parse::<i16>().unwrap();
      let order_id = Uuid::parse_str(order_id_raw);
      assert!(order_id.is_ok());
      assert_eq!(order_id.unwrap().get_version_num(),4);
      assert_eq!(location, format!("/pizza/order/{}", order_id.unwrap()));
   }

   #[test]
   fn pizza_order_test_wrong_name() {
      let rocket = rocket::ignite().mount("/", routes![super::show_menu,super::order_pizza]);
      let mut req = MockRequest::new(Method::Post, "/pizza/order")
                        .header(ContentType::Form)
                        .body(&format!("name={}", "peppppeerrroni"));
      let response = req.dispatch_with(&rocket);
      assert_eq!(response.status(), Status::NotFound);

      assert!(response.headers().find(|h| h.name() == "Location").is_none());
   }
}
