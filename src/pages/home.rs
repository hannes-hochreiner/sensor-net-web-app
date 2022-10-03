use crate::{agents::fetcher, components::NavBar, objects::sensor::Sensor};
use yew::{prelude::*, Html};
use yew_agent::{Bridge, Bridged};

pub struct HomePage {
    fetcher: Box<dyn Bridge<fetcher::Fetcher>>,
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
                    _ => Ok(()),
                },
                _ => Ok(()),
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
        };

        obj.fetcher.send(fetcher::Request::GetSensors);

        obj
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match self.process_update(ctx, msg) {
            _ => true,
        }
    }
}
