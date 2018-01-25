#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![feature(proc_macro)]

extern crate rocket;
extern crate rocket_contrib;
extern crate sled;
extern crate maud;

#[macro_use]
extern crate serde_derive;
extern crate serde;


use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use serde::{Serialize};


use maud::{html, Markup};
use rocket::State;
use rocket::response::status;
use rocket::response::NamedFile;

use rocket_contrib::Json;


fn main() {
    let path = String::from("data.db");
    let tree = sled::Config::default().path(path).tree();
    let db_arc = Arc::new(tree);

    let routes = routes![index, static_file];
    rocket::ignite().mount("/", routes).manage(db_arc).launch();
}


#[get("/")]
fn index(db: State<Arc<sled::Tree>>) -> Markup {
    // maud macro
    html! { 
        link rel="stylesheet" href="static/styles.css" {}
        body {}
        // yew-generated javascript attaches to <body>
        script src=("static/ui.js") {}
    }
}


#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

#[post("/task", data = "<task>")]
fn create_task(
    db: State<Arc<sled::Tree>>, 
    task: Json<Task>) -> status::Accepted<String> {
    println!("got a task {:?}", task);

    status::Accepted(Some(format!("success")))
}
