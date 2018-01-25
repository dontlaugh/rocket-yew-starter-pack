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
use std::path::{Path, PathBuf};

use maud::{html, Markup};
use rocket::State;
use rocket::response::NamedFile;


fn main() {
    let path = String::from("data.db");
    let tree = sled::Config::default().path(path).tree();
    let db_arc = Arc::new(tree);

    let routes = routes![index, static_file];
    rocket::ignite().mount("/", routes).manage(db_arc).launch();
}


#[get("/")]
fn index(db: State<Arc<sled::Tree>>) -> Markup {
    html! { 
        // yew requires body for rendering
        body {}
        script src=("static/ui.js") {}
    }
}


#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
