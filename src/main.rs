#![feature(proc_macro_hygiene, decl_macro)]

extern crate rust_blog;
extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
extern crate rocket_contrib;


// Rust Usage
// use std::collections::HashMap;

// Diesel Usage
use diesel::prelude::*;

// Library Usage
use rust_blog::*;
use rust_blog::schema::posts;
use rust_blog::models::{Post};

// Rocket Usage
use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};
use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<Post>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    let connection = establish_connection();
    let posts = posts::table.load::<Post>(&connection)
        .expect("Failed to load posts");
    Template::render("index", &TemplateContext {
        title: "Post Index",
        name: None,
        items: posts,
        parent: "layout"
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
