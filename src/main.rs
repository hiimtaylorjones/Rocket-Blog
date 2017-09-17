#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rust_blog;
extern crate rocket;

use rust_blog::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let connection = establish_connection();
    rocket::ignite().mount("/", routes![index]).launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn index() {
        let rocket = rocket::ignite().mount("/", routes![super::index]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
