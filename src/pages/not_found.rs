use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonSize};

pub struct NotFound {}

impl Component for NotFound {
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
            <main class="container">
                    <h1>{"Page not found"}</h1>
                    <p class="lead">
                        {"Uh oh,"}
                        <br/>
                        {"You seemed misguided, here take this wooden stick to help you on your journey"}
                    </p>
                    <Button size={ButtonSize::Small}>{"test"}</Button>
            </main>
        }
    }
}
