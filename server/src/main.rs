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

use std::sync::{Arc, Mutex};


fn main() {
    let path = String::from("data.db");
    let tree = sled::Config::default().path(path).tree();
    let db_arc = Arc::new(tree);
}

