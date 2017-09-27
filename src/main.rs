#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rust_blog;
extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use diesel::prelude::*;
use rust_blog::*;
use rust_blog::schema::posts;
use rust_blog::models::{Post};
use rocket_contrib::Template;
// use rocket::response::Redirect;
// use rocket::request::Form;

mod static_files;

#[get("/")]
fn index() -> Template {
    let connection = establish_connection();
    let posts = posts::table.load::<Post>(&connection)
        .expect("Failed to load posts");
    Template::render("index", posts)
}

fn main() {
    rocket::ignite().mount("/",
        routes![index, static_files::all]
        ).launch();
}
