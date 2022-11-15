use crate::components::navigation::Navigation;
use crate::routes::router::Route;
use yew::{function_component, html, Children, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct SidebarTemplateProps {
    pub children: Children,
}

#[function_component(SidebarTemplate)]
pub fn sidebar_template(props: &SidebarTemplateProps) -> Html {
    html! {
        <div class="flex flex-row">
            <div class="fixed top-0 left-0 h-screen w-16">
                <Navigation active_link={Route::Profile} />
            </div>
            <div class="flex">
                {for props.children.iter()}
            </div>
        </div>
    }
}
