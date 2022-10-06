use std::collections::HashSet;

use crate::{
    agents::fetcher,
    objects::{
        equipment::Equipment,
        measurement_data::{self, MeasurementData},
        sensor::Sensor,
    },
};

use super::router::AppRoute;
use uuid::Uuid;
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

pub struct Overview {
    fetcher: Box<dyn Bridge<fetcher::Fetcher>>,
    equipment: Option<Vec<Equipment>>,
    sensors: Option<Vec<Sensor>>,
    measurement_data: Option<Vec<MeasurementData>>,
}
pub enum Message {
    FetcherMessage(fetcher::Response),
}

impl Overview {
    fn process_update(&mut self, _ctx: &Context<Self>, msg: Message) -> anyhow::Result<()> {
        match msg {
            Message::FetcherMessage(msg) => match msg {
                fetcher::Response::Equipment(Ok(equipment)) => {
                    self.equipment = Some(equipment);
                    Ok(())
                }
                fetcher::Response::Sensors(Ok(sensors)) => {
                    self.sensors = Some(sensors);
                    Ok(())
                }
                fetcher::Response::MeasurementData(Ok(measurement_data)) => {
                    self.measurement_data = Some(measurement_data);
                    Ok(())
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}

impl Component for Overview {
    type Message = Message;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> VNode {
        let content = match (&self.equipment, &self.measurement_data) {
            (Some(equipment), Some(measurement_data)) => {
                html! {
                    <div>
                        {equipment.iter().map(|equip| {
                            html! {
                                <div class="card">
                                    <div class="card-header">
                                        <p class="card-header-title">{equip.info.clone().unwrap_or(equip.id.clone())}</p>
                                    </div>
                                    <div class="content">
                                        {measurement_data.iter().filter(|md| md.equipment_db_id == equip.db_id).map(|md| {
                                            html! {
                                                <p>{md.value}</p>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                }
            }
            _ => html! {},
        };
        html! {
            <section class="section">
                <h1 class="title">{"Overview"}</h1>
                {content}
            </section>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        let mut obj = Self {
            fetcher: fetcher::Fetcher::bridge(ctx.link().callback(Message::FetcherMessage)),
            equipment: None,
            sensors: None,
            measurement_data: None,
        };

        obj.fetcher.send(fetcher::Request::GetEquipment);
        obj.fetcher.send(fetcher::Request::GetSensors);
        obj.fetcher.send(fetcher::Request::GetMeasurementData);

        obj
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match self.process_update(ctx, msg) {
            Ok(_) => true,
            Err(_) => true,
        }
    }
}
