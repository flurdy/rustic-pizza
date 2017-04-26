use super::mount_rocket;
use rocket::testing::MockRequest;
use rocket::http::{ContentType, Method, Status};
use uuid::Uuid;

#[test]
fn pizza_menu_test() {
   let rocket = mount_rocket();
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
   let rocket = mount_rocket();
   let mut req = MockRequest::new(Method::Post, "/pizza/order")
                    .header(ContentType::Form)
                    .body(&format!("name={}", "pePPeroni"));
   let response = req.dispatch_with(&rocket);
   assert_eq!(response.status(), Status::SeeOther);

   let header = response.headers().find(|h| h.name() == "Location").unwrap();
   let location = header.value();
   let order_id_raw = location.split("/").last().unwrap();
   let order_id = Uuid::parse_str(order_id_raw);
   assert!(order_id.is_ok());
   assert_eq!(order_id.unwrap().get_version_num(),4);
   assert_eq!(location, format!("/pizza/order/{}", order_id.unwrap()));
}

#[test]
fn pizza_order_test_wrong_name() {
   let rocket = mount_rocket();
   let mut req = MockRequest::new(Method::Post, "/pizza/order")
                    .header(ContentType::Form)
                    .body(&format!("name={}", "peppppeerrroni"));
   let response = req.dispatch_with(&rocket);
   assert_eq!(response.status(), Status::NotFound);

   assert!(response.headers().find(|h| h.name() == "Location").is_none());
}

#[test]
fn show_pizza_ordered_test() {
   let rocket = mount_rocket();
   let order_id = Uuid::new_v4().to_string();
   let mut req = MockRequest::new(Method::Get, format!("/pizza/order/{}", order_id));
   let response = req.dispatch_with(&rocket);
   assert_eq!(response.status(), Status::Ok);
}

#[test]
fn show_pizza_ordered_test_invalid_id() {
   let rocket = mount_rocket();
   let mut req = MockRequest::new(Method::Get, "/pizza/order/123");
   let response = req.dispatch_with(&rocket);
   assert_eq!(response.status(), Status::NotFound);
}
