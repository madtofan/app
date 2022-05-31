use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    contact::ContactPage, not_found::NotFound, profile::ProfilePage, projects::ProjectsPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Profile,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Profile => html! {<ProfilePage />},
        Route::Projects => html! {<ProjectsPage/>},
        Route::Contact => html! {<ContactPage/>},
        Route::NotFound => html! {<NotFound />},
    }
}
