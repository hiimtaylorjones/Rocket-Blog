#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rust_blog;
extern crate rocket;
extern crate diesel;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use diesel::prelude::*;
use rust_blog::*;
use rust_blog::schema::posts;
use rust_blog::models::{Post};
use rocket_contrib::Template;
// use rocket::response::Redirect;
// use rocket::request::Form;

#[get("/")]
fn index() -> Template {
    let connection = establish_connection();
    let posts = posts::table.load::<Post>(&connection)
        .expect("Failed to load posts");
    Template::render("index", &posts)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
