mod components;
mod pages;
mod routes;
use components::navigation::Navigation;
use routes::router::{switch, Route};
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
            <div class="container">
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
