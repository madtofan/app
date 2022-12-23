use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{
    contacts::ContactPage, login::LoginPage, not_found::NotFound, profile::ProfilePage,
    projects::ProjectsPage, register::RegisterPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Profile,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Profile => html! {<ProfilePage />},
        Route::Login => html! {<LoginPage />},
        Route::Projects => html! {<ProjectsPage/>},
        Route::Contact => html! {<ContactPage/>},
        Route::NotFound => html! {<NotFound />},
        Route::Register => html! {<RegisterPage />},
    }
}
