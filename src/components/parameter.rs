use chrono::{DateTime, FixedOffset};
use uuid::Uuid;
use yew::{html, Component, Context, Html, Properties};

use crate::objects::parameter_type::ParameterType;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub parameter_type: ParameterType,
    pub sensor_db_id: Uuid,
    pub equipment_db_id: Uuid,
    pub value: f64,
    pub ts: DateTime<FixedOffset>,
    pub rssi: f64,
}

pub struct Parameter;

impl Component for Parameter {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Parameter
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="box">
                <nav class="level">
                    <div class="level-item has-text-centered">
                        <div>
                          <p class="heading">{ctx.props().parameter_type.id.clone()}</p>
                          <p class="title">{ctx.props().value as i64} {ctx.props().parameter_type.unit.clone()}</p>
                          <p>{ctx.props().ts.format("%Y-%m-%d %H:%M")}<br/>{format!("RSSI: {}", ctx.props().rssi)}</p>
                        </div>
                    </div>
                </nav>
            </div>
        }
    }
}
