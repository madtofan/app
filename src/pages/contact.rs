use crate::{components::navigation::Navigation, routes::router::Route};
use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonSize};

pub struct ContactContent {}

impl Component for ContactContent {
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
                <h1>{"Contact me!"}</h1>
                <p class="lead">
                    {"You can reach me via"}
                    <br/>
                    {"Email: ahmadclab@gmail.com"}
                </p>
                <Button size={ButtonSize::Small}>{"test"}</Button>
            </main>
        }
    }
}

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    html! {
        <>
            <Navigation active_link={Route::Contact}/>
            <ContactContent />
        </>
    }
}
