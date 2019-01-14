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
#[get("/index.html")]
fn index(conn:PgPool) -> Template {
    let mut context = Context::new();

    let stmt = conn.prepare("SELECT DISTINCT anlagen_system, count(*) FROM ex_ziw28 GROUP BY anlagen_system ORDER BY 1").unwrap();
    let systems:Vec<models::System> = stmt.query(&[]).unwrap().iter().map(|r| {
        models::System {
            sysnr : r.get(0),
            count : r.get(1),
        }}).collect();

    let stmt = conn.prepare("SELECT DISTINCT zusaetzl__bemerk_3, count(*) FROM ex_ziw28 GROUP BY zusaetzl__bemerk_3 ORDER BY 1").unwrap();
    let terminplaene:Vec<models::Terminplan> = stmt.query(&[]).unwrap().iter().map(|r| {
        models::Terminplan {
            name  : r.get(0),
            count : r.get(1),
        }}).collect();

    let stmt = conn.prepare("SELECT DISTINCT technische_identnummer, count(*) FROM ex_ziw28 GROUP BY technische_identnummer ORDER BY 1").unwrap();
    let equipments:Vec<models::Equipment> = stmt.query(&[]).unwrap().iter().map(|r| {
        models::Equipment {
            strukturnr  : r.get(0),
            count : r.get(1),
        }}).collect();

    let stmt = conn.prepare(models::Meldung::sql()).unwrap();
    let meldungen:Vec<models::Meldung> = stmt.query(&[]).unwrap().iter().map(|r| { models::Meldung::from_row(r) }).collect();

    context.insert("systems",&systems);
    context.insert("terminplaene", &terminplaene);
    context.insert("equpments", &equipments);
    context.insert( "meldungen",&meldungen);
    context.insert( "meldungen_json", &serde_json::to_string(&meldungen).unwrap());
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
        mount("/", routes![default,json,index,scope,files]).launch();
}
