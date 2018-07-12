use failure::Error;
use std::str::FromStr;
use yew::callback::Callback;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
#[derive(Deserialize, Debug, Default, Clone)]
pub struct Entry {
    description: String,
    completed: bool,
    // When editing true, set "editing" class, also li becomes input field
    editing: bool,
}

pub struct BackendService {
    api: FetchService,
}

impl BackendService {
    pub fn new() -> Self {
        BackendService {
            api: FetchService::new(),
        }
    }

    pub fn add(&mut self, entry: Entry, cb: Callback<Result<String, Error>>) -> FetchTask {
        let url = format!("http://localhost:8000/task");
        let handler = move |resp: Response<Json<Result<String, Error>>>| {
            let (meta, Json(result)) = resp.into_parts();
            if meta.status.is_success() {
                // This is an Ok variant?
                cb.emit(result)
            } else {
                // format_err! is a failure macro
                cb.emit(Err(format_err!("error: {:?}", result)))
            }
        };

        //let req = Request::post(url.as_str()).body(&entry).unwrap();
        let req = Request::post(url.as_str())
            .body(String::from_str("fuckthis").unwrap())
            .unwrap();
        self.api.fetch(req, handler.into())
    }
}
