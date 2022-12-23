use crate::components::templates::sidebar_template::SidebarTemplate;
use yew::prelude::*;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    html! {
        <SidebarTemplate>
            <div class="flex-grow justify-center items-center w-max h-max">
                <h1>{"Contact me!"}</h1>
                <p class="lead">
                    {"You can reach me via"}
                    <br/>
                    {"Email: ahmadclab@gmail.com"}
                </p>
                <button type="button">{"test"}</button>
            </div>
        </SidebarTemplate>
    }
}
