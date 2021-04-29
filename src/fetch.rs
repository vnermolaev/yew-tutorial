use anyhow::{anyhow, Error};
use serde::Deserialize;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::{ConsoleService, FetchService};
use yew::Callback;

pub fn get_ip_info(callback: Callback<Result<HostIPResponse, Error>>) -> FetchTask {
    let url = format!("http://api.hostip.info/get_json.php");

    ConsoleService::info(format!("Fetching: {:?}", url).as_str());

    let handler = move |response: Response<Json<_>>| {
        let (meta, Json(data)) = response.into_parts();
        if meta.status.is_success() {
            callback.emit(data)
        } else {
            callback.emit(Err(anyhow!(
                "{}: error getting ip from http://api.hostip.info/get_json.php",
                meta.status
            )))
        }
    };

    let request = Request::get(url.as_str()).body(Nothing).unwrap();

    FetchService::fetch(request, handler.into()).expect("Fetch task must be built")
}

#[derive(Deserialize, Debug)]
pub struct HostIPResponse {
    pub country_name: String,
    pub country_code: String,
    pub city: String,
    pub ip: String,
}
