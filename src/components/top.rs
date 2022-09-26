use super::router::Router;
use yew::{html, Component, Context};
use yew_router::BrowserRouter;

pub struct Top {}

pub enum Message {}

impl Component for Top {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        html! {
            <BrowserRouter>
                <Router/>
            </BrowserRouter>
        }
    }
}
