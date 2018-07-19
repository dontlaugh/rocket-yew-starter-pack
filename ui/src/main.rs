extern crate ui;
extern crate yew;

use yew::prelude::*;
use ui::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

