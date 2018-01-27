#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![feature(proc_macro)]

extern crate bincode;
extern crate rocket;
extern crate rocket_contrib;
extern crate sled;
extern crate maud;
extern crate tempdir;

#[macro_use]
extern crate serde_derive;
extern crate serde;


use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use serde::{Serialize};
use bincode::{deserialize, serialize, Infinite};


use maud::{html, Markup};
use rocket::State;
use rocket::response::status;
use rocket::response::NamedFile;

use rocket_contrib::Json;


fn main() {
    let path = String::from("data.db");
    let tree = sled::Config::default().path(path).tree();
    let db_arc = Arc::new(tree);
    let routes = routes![index, static_file, ugly_hack, create_task];
    rocket::ignite().mount("/", routes).manage(db_arc).launch();
}

fn all_routes() -> Vec<rocket::Route> {
    routes![index, static_file, ugly_hack, create_task]
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

// TODO: remove this when we figure out how to change the native Rust WebAssembly's generated
// JavaScript code to point at "static/" prefix.
#[get("/ui.wasm")]
fn ugly_hack() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/ui.wasm")).ok()
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    completed: bool,
    description: String,
}


#[post("/task", format = "application/json", data = "<task>")]
fn create_task(
    db: State<Arc<sled::Tree>>,
    task: Json<Task>) -> status::Accepted<String> {
    println!("got a task {:?}", task);

    let mut count = 0;
    for (_, _) in db.iter() {
        count += 1;
    }
    let new_key = vec![count];
    // our task is the first field "0" on a Json<Task> rocket passes us.
    // Our task implements Serialize, required here.
    let encoded: Vec<u8> = serialize(&task.0, Infinite).unwrap();
    db.set(new_key, encoded);
    status::Accepted(Some(format!("success")))
}


fn test_instance(path: PathBuf) -> rocket::Rocket {
    let tree = sled::Config::default()
        .path(String::from(path.to_str().unwrap()))
        .tree();
    let db_arc = Arc::new(tree);
    rocket::ignite().mount("/", all_routes()).manage(db_arc)
}

#[test]
fn test_post_get() {
    use tempdir::TempDir;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    let dir = TempDir::new("rocket").unwrap();
    let path = dir.path().join("test_data.db");
    
    let c = Client::new(test_instance(path)).unwrap();
    let req = c.post("/task")
        .body(r#"{"completed": false, "description": "foo"}"#)
        .header(ContentType::JSON);
    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Accepted);
}
