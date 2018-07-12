#![recursion_limit = "128"]
#![feature(proc_macro)]
#![feature(type_ascription)]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde_derive;

#[macro_use] // json!{ } ??
extern crate serde_json;

#[macro_use]
extern crate yew;

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate failure;

use std::time::Duration;

use strum::IntoEnumIterator;

use yew::format::{Json, Nothing, Text};
use yew::html::*;
use yew::prelude::*;
use yew::services::fetch::{FetchService, Request, Response, StatusCode};
use yew::services::interval::IntervalService;
use yew::services::storage::{Area, StorageService};

mod backend;

use backend::{BackendService, Entry};

const KEY: &'static str = "yew.todomvc.self";

fn main() {
    yew::initialize();
    // t-t-t-t-turboooofiissshh !!!!!!!!!!
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

struct Model {
    data: Data,
    api: BackendService,
    storage: StorageService,
    link: ComponentLink<Model>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Data {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateAll(Vec<Entry>),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    Tick,
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: ComponentLink<Self>) -> Self {
        // Start background interval
        let mut interval = IntervalService::new();
        let interval_cb = context.send_back(|_| Msg::Tick);
        let handle = interval.spawn(Duration::from_secs(10), interval_cb);

        // fetch the canonical state from the server...
        let fetch_cb = context.send_back(|resp: Response<Json<Result<String, failure::Error>>>| {
            let (meta, Json(result)) = resp.into_parts();
            if meta.status.is_success() {
                let real_result: Vec<Entry> = serde_json::from_str(&result.unwrap()).unwrap();
                Msg::UpdateAll(real_result)
            } else {
                js! { console.log("fetching all tasks failed") };
                Msg::Nope
            }
        });
        let mut storage = StorageService::new(Area::Local);
        let mut fetcher = FetchService::new();
        let req = Request::get("http://localhost:8000/tasks")
            .body(Nothing)
            .unwrap();

        fetcher.fetch(req, fetch_cb);

        // ...but load from local storage until we fire the callback
        if let Json(Ok(restored)) = storage.restore(KEY) {
            Model {
                data: restored,
                api: BackendService::new(),
                storage: storage,
                link: context,
            }
        } else {
            Model {
                data: Data {
                    entries: Vec::new(),
                    filter: Filter::All,
                    value: "".into(),
                    edit_value: "".into(),
                },
                api: BackendService::new(),
                storage: storage,
                link: context,
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    description: self.data.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.data.entries.push(entry.clone());
                self.data.value = "".to_string();
                // trying strategy: https://github.com/DenisKolodin/yew/issues/179
                // let emitter = self.link.send_back(function)a;
                let cb = self.link.send_back(
                    move |resp: Response<Json<Result<String, failure::Error>>>| {
                        let (meta, Json(body)) = resp.into_parts();
                        if meta.status.is_success() {
                            println!("success");
                            Msg::Nope
                        } else {
                            println!("fail");
                            Msg::Nope
                        }
                    },
                );
                //let body = serde_json::to_string(&entry).unwrap();
                let req = Request::post("http://localhost:8000/task")
                    .header("Content-Type", "application/json")
                    .body(&entry)
                    .expect("could not build request");
                self.fetch_service.fetch(req, cb);
            }
            Msg::Edit(idx) => {
                let edit_value = self.data.edit_value.clone();
                self.complete_edit(idx, edit_value);
                self.data.edit_value = "".to_string();
            }
            Msg::Tick => {
                let entries = self.data.entries.clone();
                let cb = self.link.send_back(
                    move |resp: Response<Json<Result<serde_json::Value, failure::Error>>>| {
                        let code: StatusCode = resp.status();
                        if code.is_success() {
                            js! { console.log("sync success") };
                            Msg::Nope
                        } else {
                            Msg::Nope
                        }
                    },
                );
                let body = serde_json::to_string(&entries).unwrap();
                let req = Request::post("http://localhost:8000/tasks")
                    .header("Content-Type", "application/json")
                    .body(Json(json!(body)))
                    //.body(body)
                    .expect("could not build request");
                self.fetch_service.fetch(req, cb.into());
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                js! { console.log("input:", @{val.clone()}) };
                self.data.value = val;
            }
            Msg::UpdateAll(vals) => {
                js! { console.log("got all tasks:") };
                self.data.entries = vals;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.data.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.data.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.data.edit_value = self.data.entries[idx].description.clone();
                self.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.is_all_completed();
                self.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.clear_completed();
            }
            Msg::Nope => {}
        }
        // Our internal data field is serializable as JSON, and we store commit data
        // to local storage on every update.
        self.storage.store(KEY, Json(&self.data));
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="todomvc-wrapper",>
                <section class="todoapp",>
                    <header class="header",>
                        <h1>{ "todos!" }</h1>
                        { self.view_input() } // make new thing
                    </header>
                    <section class="main",>
                        <input class="toggle-all", type="checkbox", checked=self.is_all_completed(), onclick=|_| Msg::ToggleAll, />
                        <ul class="todo-list",>
                            { for self.data.entries.iter().filter(|e| self.data.filter.fit(e)).enumerate().map(view_entry) }
                        </ul>
                    </section>
                    <footer class="footer",>
                        <span class="todo-count",>
                            <strong>{ self.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters",>
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                        <button class="clear-completed", onclick=|_| Msg::ClearCompleted,>
                            { format!("Clear completed ({})", self.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info",>
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/DenisKolodin/", target="_blank",>{ "Denis Kolodin" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/", target="_blank",>{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Html<Model> {
        let flt = filter.clone();
        html! {
            <li>
                <a class=if self.data.filter == flt { "selected" } else { "not-selected" },
                   href=&flt,
                   onclick=|_| Msg::SetFilter(flt.clone()),>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html<Model> {
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input class="new-todo",
                   placeholder="What needs to be done?",
                   value=&self.data.value,
                   oninput=|e| Msg::Update(e.value),
                   onkeypress=|e| {
                       if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                   }, />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }
}

fn view_entry((idx, entry): (usize, &Entry)) -> Html<Model> {
    html! {
        <li class=if entry.editing == true { "editing" } else { "" },>
            <div class="view",>
                <input class="toggle", type="checkbox", checked=entry.completed, onclick=|_| Msg::Toggle(idx), />
                <label ondoubleclick=|_| Msg::ToggleEdit(idx),>{ &entry.description }</label>
                <button class="destroy", onclick=|_| Msg::Remove(idx), />
            </div>
            { view_entry_edit_input((idx, &entry)) }
        </li>
    }
}

fn view_entry_edit_input((idx, entry): (usize, &Entry)) -> Html<Model> {
    if entry.editing == true {
        html! {
            <input class="edit",
                   type="text",
                   value=&entry.description,
                   oninput=|e| Msg::UpdateEdit(e.value),
                   onblur=|_| Msg::Edit(idx),
                   onkeypress=|e| {
                      if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                   }, />
        }
    } else {
        html! { <input type="hidden", /> }
    }
}

pub struct TodoService {
    api: FetchService,
}

impl TodoService {
    pub fn new() -> Self {
        Self {
            api: FetchService::new(),
        }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
}

impl Model {
    fn total(&self) -> usize {
        self.data.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.data
            .entries
            .iter()
            .filter(|e| Filter::Completed.fit(e))
            .count()
    }

    fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .data
            .entries
            .iter()
            .filter(|e| self.data.filter.fit(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.completed)
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.data.entries.iter_mut() {
            if self.data.filter.fit(entry) {
                entry.completed = value;
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self
            .data
            .entries
            .drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.data.entries = entries;
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.data.filter.clone();
        let mut entries = self
            .data
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }

    fn toggle_edit(&mut self, idx: usize) {
        let filter = self.data.filter.clone();
        let mut entries = self
            .data
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.editing = !entry.editing;
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let filter = self.data.filter.clone();
        let mut entries = self
            .data
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.description = val;
        entry.editing = !entry.editing;
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let filter = self.data.filter.clone();
            let entries = self
                .data
                .entries
                .iter()
                .enumerate()
                .filter(|&(_, e)| filter.fit(e))
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };
        self.data.entries.remove(idx);
    }
}
