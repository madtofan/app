use crate::components::templates::sidebar_template::SidebarTemplate;
use yew::prelude::*;

#[function_component(EditProfilePage)]
pub fn edit_profile_page() -> Html {
    html! {
        <SidebarTemplate>
            {"Edit profile"}
        </SidebarTemplate>
    }
}
