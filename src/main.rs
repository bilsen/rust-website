
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(rustc_private)]
#![allow(dead_code)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate log;



use rocket_contrib::{
    templates::Template,
    serve::StaticFiles,
};

#[database("psql_database")]
pub struct LogsDbConn(diesel::PgConnection);


mod schema;
mod db;
mod error;
//mod login;
mod requests;
mod page_context;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            requests::login::login,
            requests::login::submit_login,
            requests::login::submit_registration,
            requests::home::home,
        ])
        .mount("/public/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .attach(LogsDbConn::fairing())
        .launch();
}