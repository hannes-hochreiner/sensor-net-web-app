use super::notifier;
use crate::objects::{sensor::Sensor, JsError};
use crate::utils;
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::ConnectionType;
use yew_agent::{Agent, AgentLink, Context, Dispatched, Dispatcher, HandlerId};

#[derive(Debug)]
pub enum Request {
    GetSensors,
}

#[derive(Debug)]
pub enum Response {
    Sensors(Result<Vec<Sensor>, JsError>),
}

#[derive(Debug)]
pub enum Message {
    ReceiveSensors(HandlerId, Result<Vec<Sensor>, JsError>),
}

pub struct Fetcher {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    notifier: Dispatcher<notifier::Notifier>,
}

enum HttpMethod {
    Get,
    Post,
}

impl Fetcher {
    fn process_update(&mut self, msg: Message) -> Result<(), JsError> {
        match msg {
            Message::ReceiveSensors(id, res) => self.link.respond(id, Response::Sensors(res)),
        }

        Ok(())
    }

    fn process_handle_input(&mut self, msg: Request, id: HandlerId) -> Result<(), JsError> {
        let conn_type = utils::get_connection_type()?;

        if (conn_type != ConnectionType::Ethernet)
            & (conn_type != ConnectionType::Wifi)
            & (conn_type != ConnectionType::Unknown)
        {
            return Ok(());
        }

        match msg {
            Request::GetSensors => self.link.send_future(async move {
                Message::ReceiveSensors(
                    id,
                    fetch_deserializable("/api/sensors", HttpMethod::Get, None, None).await,
                )
            }),
        }

        Ok(())
    }
}

impl Agent for Fetcher {
    type Reach = Context<Self>;
    type Message = Message;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::<HandlerId>::new(),
            notifier: notifier::Notifier::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match self.process_update(msg) {
            Ok(_) => {}
            Err(e) => self.notifier.send(notifier::Request::NotifyError(e)),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match self.process_handle_input(msg, id) {
            Ok(_) => {}
            Err(e) => self.notifier.send(notifier::Request::NotifyError(e)),
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

fn encode_query_pairs(pairs: &Vec<(&str, &str)>) -> String {
    let mut tmp = url::form_urlencoded::Serializer::new(String::new());

    for (key, value) in pairs {
        tmp.append_pair(key, value);
    }

    tmp.finish()
}

async fn fetch(
    url: &str,
    method: HttpMethod,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> Result<web_sys::Response, JsError> {
    let mut opts = web_sys::RequestInit::new();

    match method {
        HttpMethod::Get => opts.method("GET"),
        HttpMethod::Post => opts.method("POST"),
    };

    if let Some(headers) = headers {
        let opt_headers = web_sys::Headers::new()?;

        for (key, val) in headers {
            opt_headers.append(&key, &val)?;
        }

        opts.headers(&opt_headers);
    }

    if let Some(val) = body {
        opts.body(Some(&serde_wasm_bindgen::to_value(&val)?));
    }

    let request = web_sys::Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or("error getting window")?;
    let resp: web_sys::Response = JsFuture::from(window.fetch_with_request(&request))
        .await?
        .dyn_into()?;

    match resp.ok() {
        true => Ok(resp),
        false => {
            Err((&*format!("fetcher error: {}: {}", resp.status(), resp.status_text())).into())
        }
    }
}

async fn fetch_deserializable<T: DeserializeOwned>(
    url: &str,
    method: HttpMethod,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> Result<T, JsError> {
    JsFuture::from(fetch(url, method, headers, body).await?.json()?)
        .await
        .map(|val| serde_wasm_bindgen::from_value(val).map_err(Into::into))?
}
