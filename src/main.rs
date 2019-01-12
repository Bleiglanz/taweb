#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;
extern crate serde;
extern crate taweb;
extern crate dotenv;

use rocket_contrib::templates::Template;
use rocket_contrib::databases::diesel;
use tera::Context;
use dotenv::dotenv;

#[database("taweb")]
struct PgPool(diesel::PgConnection);
fn conn(pool: &diesel::PgConnection) -> &diesel::PgConnection { pool }



#[get("/")]
fn index(pool:PgPool) -> Template {
    let conn = conn(&pool);

    use taweb::schema::systems::dsl::*;
    use taweb::models::Systems;
    use diesel::RunQueryDsl;

    let systems_list = systems.load::<Systems>(conn)
        .expect("Error loading systems");

    let mut context = Context::new();
    context.insert("systems",&systems_list);
    Template::render("index",&context)
}

#[get("/scope")]
fn scope() -> Template {
    let mut context = Context::new();
    Template::render("scope",&context)
}

#[get("/files")]              // <- route attribute
fn files() -> Template {
    let mut context = Context::new();
    Template::render("files",&context)
}

fn main() {
    dotenv().ok();
    rocket::ignite().
        attach(Template::fairing()).
        attach(PgPool::fairing()).
        mount("/", routes![index,scope,files]).launch();
}