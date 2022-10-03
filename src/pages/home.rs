use std::collections::HashSet;

use crate::{
    agents::fetcher,
    components::NavBar,
    objects::{equipment::Equipment, parameter_type::ParameterType, sensor::Sensor},
};
use uuid::Uuid;
use yew::{prelude::*, Html};
use yew_agent::{Bridge, Bridged};

pub struct HomePage {
    fetcher: Box<dyn Bridge<fetcher::Fetcher>>,
    sensors: Option<HashSet<Uuid, Sensor>>,
    equipment: Option<HashSet<Uuid, Equipment>>,
    parameter_types: Option<HashSet<Uuid, ParameterType>>,
}

pub enum Message {
    FetcherMessage(fetcher::Response),
}

impl HomePage {
    fn process_update(&mut self, _ctx: &Context<Self>, msg: Message) -> anyhow::Result<()> {
        match msg {
            Message::FetcherMessage(resp) => match resp {
                fetcher::Response::Sensors(res) => match res {
                    Ok(sensors) => {
                        log::debug!("{:?}", sensors);
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                },
                fetcher::Response::Equipment(res) => match res {
                    Ok(equipment) => {
                        log::debug!("{:?}", equipment);
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                },
                fetcher::Response::ParameterTypes(res) => match res {
                    Ok(parameter_types) => {
                        log::debug!("{:?}", parameter_types);
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                },
            },
        }
    }
}

impl Component for HomePage {
    type Message = Message;
    type Properties = ();

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <NavBar/>
                // <Notification/>
                // <Player/>
            </>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        let mut obj = Self {
            fetcher: fetcher::Fetcher::bridge(ctx.link().callback(Message::FetcherMessage)),
            equipment: None,
            parameter_types: None,
            sensors: None,
        };

        obj.fetcher.send(fetcher::Request::GetEquipment);
        obj.fetcher.send(fetcher::Request::GetSensors);
        obj.fetcher.send(fetcher::Request::GetParameterTypes);

        obj
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match self.process_update(ctx, msg) {
            _ => true,
        }
    }
}
