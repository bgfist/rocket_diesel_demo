#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

mod controllers;
mod models;
mod repositories;
mod schema;

use rocket::routes;
use rocket_contrib::database;

#[database("self_crm")]
struct CrmDbConn(diesel::MysqlConnection);

fn main() {
    use controllers::*;

    rocket::ignite()
        .mount("/account", routes![account::list])
        .attach(CrmDbConn::fairing())
        .launch();
}
