#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;
extern crate serde;
extern crate taweb;
extern crate dotenv;
extern crate postgres;

mod models;

use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use rocket::http::RawStr;
use tera::Context;
use dotenv::dotenv;

#[database("taweb")]
struct PgPool(postgres::Connection);

#[get("/")]
fn default() -> Redirect {
    Redirect::to("/index.html")
}

#[get("/json/<name>/data.js", format="json")]
fn json(conn:PgPool, name: &RawStr) -> Json<Vec<models::Meldung>> {
    let stmt = conn.prepare(models::Meldung::sql()).unwrap();
    let meldungen:Vec<models::Meldung> = stmt.query(&[]).unwrap().iter().map(|r| { models::Meldung::from_row(r) }).collect();
    Json(meldungen)
}
#[get("/json/<name>/header.js", format="json")]
fn json_header(conn:PgPool, name: &RawStr) -> Json<Vec<String>> {
    let stmt = conn.prepare(models::Meldung::sql()).unwrap();
    let cols = stmt.columns();
    let headers:Vec<String> = cols.iter().map(|c| { c.name().to_string() }).collect();
    Json(headers)
}


#[get("/index.html")]
fn index() -> Template {
    let mut context = Context::new();
    Template::render("index",&context)
}

#[get("/scope.html")]
fn scope() -> Template {
    let context = Context::new();
    Template::render("scope",&context)
}

#[get("/files.html")]              // <- route attribute
fn files() -> Template {
    let context = Context::new();
    Template::render("files",&context)
}

fn main() {
    dotenv().ok();
    rocket::ignite().
        attach(Template::fairing()).
        attach(PgPool::fairing()).
        mount("/", routes![default,json,index,scope,files,json_header]).launch();
}
