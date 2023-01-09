use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{
    auth::{login::LoginPage, register::RegisterPage},
    contacts::ContactPage,
    profile::{edit_profile::EditProfilePage, profile::ProfilePage},
    projects::ProjectsPage,
    NotFound,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    // #[at("/")]
    // Profile,
    #[at("/profile")]
    Profile,
    #[at("/profile/edit")]
    EditProfilePage,
    #[at("/auth/login")]
    Login,
    #[at("/auth/register")]
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
        Route::EditProfilePage => html! {<EditProfilePage />},
        Route::Login => html! {<LoginPage />},
        Route::Projects => html! {<ProjectsPage/>},
        Route::Contact => html! {<ContactPage/>},
        Route::NotFound => html! {<NotFound />},
        Route::Register => html! {<RegisterPage />},
    }
}
