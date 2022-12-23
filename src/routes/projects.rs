use crate::components::templates::sidebar_template::SidebarTemplate;
use yew::prelude::*;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html! {
        <SidebarTemplate>
            <div class="flex-grow justify-center items-center w-max h-max">
                <h1>{"Projects page"}</h1>
                <p class="lead">
                    {"These are the list of projects under my belt"}
                    <br/>
                    {"Note that these projects are not projects from my Day Job as I am obliged not to talk about those"}
                </p>
            </div>
        </SidebarTemplate>
    }
}
