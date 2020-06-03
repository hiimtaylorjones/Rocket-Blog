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
struct PostsTemplateContext {
    title: &'static str,
    name: Option<String>,
    posts: Vec<Post>,
    parent: &'static str,
}

#[derive(Serialize)]
struct PostTemplateContext {
    title: &'static str,
    name: Option<String>,
    post: Vec<Post>,
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    let connection = establish_connection();
    let posts = posts::table.load::<Post>(&connection)
        .expect("Failed to load posts");
    Template::render("index", &PostsTemplateContext {
        title: "Post Index",
        name: None,
        posts: posts,
        parent: "layout"
    })
}

#[get("/posts/<id>")]
fn find_post(id: i32) -> Template {
    use self::schema::posts::dsl::*;
    let connection = establish_connection();

    let post = posts.find(id)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    Template::render("show_post", &PostTemplateContext {
        title: "Post",
        name: None,
        post: post,
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
        .mount("/", routes![index, find_post])
        .register(catchers![not_found])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
