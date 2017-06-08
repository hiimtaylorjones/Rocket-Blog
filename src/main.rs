#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::testing::MockRequest;
    use rocket::http::{Status, Method};

    #[test]
    fn index() {
        let rocket = rocket::ignite().mount("/", routes![super::index]);
        let mut req = MockRequest::new(Method::Get, "/");
        let mut response = req.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);

        let body_str = response.body().and_then(|b| b.into_string());
        assert_eq!(body_str, Some("Hello, world!".to_string()));
    }
}
