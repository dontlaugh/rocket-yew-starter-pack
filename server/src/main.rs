#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![feature(proc_macro)]

extern crate rocket;
extern crate sled;

extern crate maud;


use std::sync::{Arc, Mutex};
use rocket::State;

use maud::{html, Markup};


fn main() {
    let path = String::from("data.db");
    let tree = sled::Config::default().path(path).tree();
    let db_arc = Arc::new(tree);

    let routes = routes![index];
    rocket::ignite().mount("/", routes).manage(db_arc).launch();
}


#[get("/")]
fn index(db: State<Arc<sled::Tree>>) -> Markup {
    html! { 
        script src=("ui.js") {}
    }
}

