use chrono::{format::Fixed, DateTime, Duration, FixedOffset, Utc};
use url::Url;
use uuid::Uuid;
use yew::{classes, html, Component, Context, Html, Properties};

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

pub enum Message {
    SwitchToTab(Tabs),
}

#[derive(Debug, PartialEq)]
pub enum Tabs {
    Value,
    LastHour,
    LastDay,
}

pub struct Parameter {
    current_tab: Tabs,
}

impl Component for Parameter {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Parameter {
            current_tab: Tabs::Value,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SwitchToTab(tab) => {
                self.current_tab = tab;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="box">
                <div class="tabs is-centered">
                    <ul>
                        <li onclick={ctx.link().callback(|_| Message::SwitchToTab(Tabs::Value))} class={classes!(self.is_tab_active(Tabs::Value))}><a>{"value"}</a></li>
                        <li onclick={ctx.link().callback(|_| Message::SwitchToTab(Tabs::LastHour))} class={classes!(self.is_tab_active(Tabs::LastHour))}><a>{"last hour"}</a></li>
                        <li onclick={ctx.link().callback(|_| Message::SwitchToTab(Tabs::LastDay))} class={classes!(self.is_tab_active(Tabs::LastDay))}><a>{"last day"}</a></li>
                    </ul>
                </div>
                {match self.current_tab {
                    Tabs::Value => self.show_value(ctx),
                    Tabs::LastHour => {
                        let end = Utc::now();
                        let start = Utc::now() - Duration::hours(1);
                        self.show_plot(ctx, &start.into(), &end.into())},
                    Tabs::LastDay => {
                        let end = Utc::now();
                        let start = Utc::now() - Duration::days(1);
                        self.show_plot(ctx, &start.into(), &end.into())},
                    }}
            </div>
        }
    }
}

impl Parameter {
    fn is_tab_active(&self, tab: Tabs) -> Option<String> {
        match self.current_tab == tab {
            true => Some("is-active".into()),
            false => None,
        }
    }

    fn show_value(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav class="level">
                <div class="level-item has-text-centered">
                    <div>
                        <p class="heading">{ctx.props().parameter_type.id.clone()}</p>
                        <p class="title">{ctx.props().value as i64} {ctx.props().parameter_type.unit.clone()}</p>
                        <p>{ctx.props().ts.format("%Y-%m-%d %H:%M")}<br/>{format!("RSSI: {}", ctx.props().rssi)}</p>
                    </div>
                </div>
            </nav>
        }
    }

    fn show_plot(
        &self,
        ctx: &Context<Self>,
        start: &DateTime<FixedOffset>,
        end: &DateTime<FixedOffset>,
    ) -> Html {
        let base = web_sys::window().unwrap().location().origin().unwrap();
        let url = Url::parse_with_params(
            &format!("{}/api/plot", base),
            &[
                ("startTime", start.to_rfc3339()),
                ("endTime", end.to_rfc3339()),
                ("equipmentDbId", ctx.props().equipment_db_id.to_string()),
                ("sensorDbId", ctx.props().sensor_db_id.to_string()),
                (
                    "parameterTypeDbId",
                    ctx.props().parameter_type.db_id.to_string(),
                ),
            ],
        )
        .unwrap();

        html! {
            <img src={url.to_string()}/>
        }
    }
}
