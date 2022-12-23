use crate::components::navigation::Navigation;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SidebarTemplateProps {
    pub children: Children,
}

#[function_component(SidebarTemplate)]
pub fn sidebar_template(props: &SidebarTemplateProps) -> Html {
    html! {
        <div class="flex flex-col md:flex-row">
            <div class="w-screen md:h-screen md:w-64 shadow-lg shadow-black">
                <Navigation />
            </div>
            <div class="flex-grow py-4 px-4 bg-slate-50">
                {for props.children.iter()}
            </div>
        </div>
    }
}
