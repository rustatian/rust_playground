#![feature(plugin)]
#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use rocket::config::Environment;
use rocket::logger::LoggingLevel;
use rocket::routes;
use rocket::{get, Config};
use rocket_contrib::json::Json;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
struct Timestamp {
    time: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/time")]
fn time_now() -> Json<Timestamp> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = Timestamp {
        time: now.to_rfc3339(),
    };
    Json(timestamp)
}

fn main() {
    //    let config = rocket::Config::build(Environment::Staging)
    //        .address("0.0.0.0")
    //        .log_level(LoggingLevel::Off)
    //        .port(8000)
    //        .workers(12)
    //        .finalize()
    //        .unwrap();
    //
    //    rocket::custom(config)
    //        .mount("/", routes![index, time_now])
    //        .launch();

    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));
}
