use crate::templates::sidebar_template::SidebarTemplate;
use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonSize};

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    html! {
        <SidebarTemplate>
            <div class="flex-grow justify-center items-center w-max h-max">
                <h1>{"Hi"}</h1>
                <p class="lead">
                    {"My name is Ahmad"}
                    <br/>
                    {"A Malaysian Fullstack Developer and Technology Enthusiast"}
                </p>
                <Button size={ButtonSize::Small}>{"test"}</Button>
            </div>
        </SidebarTemplate>
    }
}
