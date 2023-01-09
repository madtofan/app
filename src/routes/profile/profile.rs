use crate::components::templates::sidebar_template::SidebarTemplate;
use yew::prelude::*;

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
                <button type="button">{"test"}</button>
            </div>
        </SidebarTemplate>
    }
}
