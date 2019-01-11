#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use tera::Context;


#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("links",&"Einszwei");
    context.insert("rechts", &vec!["A","B","C","D","E"]);
    Template::render("index",&context)
}

fn main() {
    rocket::ignite().
        attach(Template::fairing()).
        mount("/", routes![index]).launch();
}
