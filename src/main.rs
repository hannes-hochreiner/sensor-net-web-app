mod agents;
mod components;
mod objects;
mod pages;
mod utils;
use components::Top;
// use yew::prelude::*;
// use wasm_bindgen::prelude::*;

// enum Msg {
//     AddOne,
// }

// struct Model {
//     value: i64,
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self { value: 0 }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::AddOne => {
//                 self.value += 1;
//                 // the value has changed so we need to
//                 // re-render for it to appear on the page
//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
//         let link = ctx.link();
//         html! {
//             <div>
//                 <nav class="navbar" role="navigation" aria-label="main navigation">
//                     <div class="navbar-menu">
//                         <div class="navbar-start">
//                             <a class="navbar-item">{"Home"}</a>
//                             <a class="navbar-item">{"Documentation"}</a>
//                         </div>
//                     </div>
//                 </nav>
//                 <button class="button" onclick={link.callback(|_| Msg::AddOne)}>
//                     <span class="material-icons">{"add"}</span>
//                 </button>
//                 <p>{ self.value }</p>
//             </div>
//         }
//     }
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // yew::start_app::<Model>();
    yew::start_app::<Top>();
}
