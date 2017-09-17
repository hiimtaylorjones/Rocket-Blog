#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rust_blog;
extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use diesel::prelude::*;
use rocket_blog::*;
use rocket_blog::schema::posts;
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;

mod static_files;

#[get("/")]
fn index() -> Template {
    let connection = establish_connection();
    let posts = posts::table.load::<Post>(&connection)
        .expect("Failed to load posts");
    let context = map!["posts" => posts];
    Template::render("index", &context)
}

#[get("/blog")]
fn blog() -> Template {
    let connection = establish_connection();
    let posts = posts::table.load::<Post>(&connection)
        .expect("Failed to load posts");
    let context = map!["posts" => posts];
    Template::render("blog", &context)
}

#[get("/meetups")]
fn meetups() -> Template {
    let connection = establish_connection();
    let context = map!["title" => "Meetups"];
    Template::render("meetups", &context)
}

#[get("/projects")]
fn projects() -> Template {
    let connection = establish_connection();
    let context = map!["title" => "Projects"];
    Template::render("projects", &context)
}

#[get("/posts/new")]
fn new_post() -> Template {
    let context = map!["title" => "New Post"];
    Template::render("new_post", &context)
}

#[post("/posts", data = "<post>")]
fn create_post(post: Form<PostForm>) -> Redirect {
    let connection = establish_connection();
    diesel::insert(post.get())
        .into(posts::table)
        .execute(&connection)
        .expect("Failed to create post");
    Redirect::found("/")
}

#[delete("/posts/<id>")]
fn delete_post(id: i64) -> Redirect {
    let connection = establish_connection();
    diesel::delete(posts::table.find(id))
        .execute(&connection)
        .expect("Failed to delete post");
    Redirect::found("/")
}

#[get("/posts/<id>/edit")]
fn edit_post(id: i64) -> Option<Template> {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        post: Post,
    }

    let connection = establish_connection();
    let post = posts::table.find(id)
        .first::<Post>(&connection)
        .optional()
        .expect("Failed to load post");
    post.map(|post| {
        let context = Context { title:  "Editing Post", post: post };
        Template::render("edit_post", &context)
    })
}

#[patch("/posts/<id>", data = "<post_data>")]
fn update_post(id: i64, post_data: Form<PostForm>) -> Redirect {
    let connection = establish_connection();
    let updated_rows = diesel::update(posts::table.find(id))
        .set(post_data.get())
        .execute(&connection)
        .expect("Failed to update post");
    assert_eq!(1, updated_rows, "Could not find a post to update");
    Redirect::found("/")
}

fn main() {
    rocket::ignite().mount("/",
        routes![index, blog, new_post, create_post,
            delete_post, edit_post, update_post,
            meetups, projects, static_files::all]
        ).launch();
}
