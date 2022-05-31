use crate::{components::navigation::Navigation, routes::router::Route};
use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonSize};

pub struct ProfileContent {}

impl Component for ProfileContent {
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
                <h1>{"Hi"}</h1>
                <p class="lead">
                    {"My name is Ahmad"}
                    <br/>
                    {"A Malaysian Fullstack Developer and Technology Enthusiast"}
                </p>
                <Button size={ButtonSize::Small}>{"test"}</Button>
            </main>
        }
    }
}

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    html! {
        <>
            <Navigation active_link={Route::Profile}/>
            <ProfileContent />
        </>
    }
}
