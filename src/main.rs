mod components;
mod hooks;
mod routes;
mod services;
mod types;
use components::features::context_provider::AppContext;
use services::router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <AppContext>
                    <Switch<Route> render={switch} />
                </AppContext>
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
