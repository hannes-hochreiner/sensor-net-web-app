use super::notifier;
use crate::objects::equipment::Equipment;
use crate::objects::measurement_data::MeasurementData;
use crate::objects::parameter_type::ParameterType;
use crate::objects::{sensor::Sensor, JsError};
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use yew_agent::{Agent, AgentLink, Context, Dispatched, Dispatcher, HandlerId};

#[derive(Debug)]
pub enum Request {
    GetSensors,
    GetEquipment,
    GetParameterTypes,
    GetMeasurementData,
}

#[derive(Debug)]
pub enum Response {
    Sensors(Result<Vec<Sensor>, JsError>),
    Equipment(Result<Vec<Equipment>, JsError>),
    ParameterTypes(Result<Vec<ParameterType>, JsError>),
    MeasurementData(Result<Vec<MeasurementData>, JsError>),
}

#[derive(Debug)]
pub enum Message {
    ReceiveSensors(HandlerId, Result<Vec<Sensor>, JsError>),
    ReceiveEquipment(HandlerId, Result<Vec<Equipment>, JsError>),
    ReceiveParameterTypes(HandlerId, Result<Vec<ParameterType>, JsError>),
    ReceiveMeasurementData(HandlerId, Result<Vec<MeasurementData>, JsError>),
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
            Message::ReceiveEquipment(id, res) => self.link.respond(id, Response::Equipment(res)),
            Message::ReceiveParameterTypes(id, res) => {
                self.link.respond(id, Response::ParameterTypes(res))
            }
            Message::ReceiveMeasurementData(id, res) => {
                self.link.respond(id, Response::MeasurementData(res))
            }
        }

        Ok(())
    }

    fn process_handle_input(&mut self, msg: Request, id: HandlerId) -> Result<(), JsError> {
        match msg {
            Request::GetSensors => self.link.send_future(async move {
                Message::ReceiveSensors(
                    id,
                    fetch_deserializable("/api/sensors", HttpMethod::Get, None, None).await,
                )
            }),
            Request::GetEquipment => self.link.send_future(async move {
                Message::ReceiveEquipment(
                    id,
                    fetch_deserializable("/api/equipment", HttpMethod::Get, None, None).await,
                )
            }),
            Request::GetParameterTypes => self.link.send_future(async move {
                Message::ReceiveParameterTypes(
                    id,
                    fetch_deserializable("/api/parameter_types", HttpMethod::Get, None, None).await,
                )
            }),
            Request::GetMeasurementData => self.link.send_future(async move {
                Message::ReceiveMeasurementData(
                    id,
                    fetch_deserializable(
                        "/api/measurement_data/latest",
                        HttpMethod::Get,
                        None,
                        None,
                    )
                    .await,
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
