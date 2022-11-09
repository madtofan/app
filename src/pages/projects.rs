use crate::{components::navigation::Navigation, routes::router::Route};
use yew::prelude::*;

pub struct ProjectsContent;

impl Component for ProjectsContent {
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
                <h1>{"Projects page"}</h1>
                <p class="lead">
                    {"These are the list of projects under my belt"}
                    <br/>
                    {"Note that these projects are not projects from my Day Job as I am obliged not to talk about those"}
                </p>
            </main>
        }
    }
}

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html! {
        <>
            <Navigation active_link={Route::Projects} />
            <ProjectsContent/>
        </>
    }
}
