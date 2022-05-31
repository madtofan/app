use crate::routes::router::Route;
use yew::prelude::*;

pub struct Navigation {}

#[derive(Clone, PartialEq, Properties)]
pub struct NavigationProps {
    pub active_link: Option<Route>,
}

impl Component for Navigation {
    type Message = ();
    type Properties = NavigationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let active_page = &ctx.props().active_link.as_ref().unwrap_or(&Route::NotFound);
        html! {
            <nav class="navbar navbar-expand-md navbar-dark bg-dark fixed-top">
                <a class="navbar-brand" href="#">{"Navbar"}</a>
                <button class="navbar-toggler">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarsExampleDefault">
                    <ul class="navbar-nav mr-auto">
                        <li class={match active_page {
                            Route::Profile => "nav-item active",
                            _ => "nav-item"
                        }}>
                            <a class="nav-link" href="/">
                                {"Home "}
                            </a>
                        </li>
                        <li class={match active_page {
                            Route::Projects => "nav-item active",
                            _ => "nav-item"
                        }}>
                        <a class="nav-link" href="/projects">
                            {"Project "}
                        </a>
                        </li>
                        <li class={match active_page {
                            Route::Contact => "nav-item active",
                            _ => "nav-item"
                        }}>
                        <a class="nav-link" href="/contact">
                            {"Contact "}
                        </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link disabled" href="#">{"Disabled"}</a>
                        </li>
                    </ul>
                    // <form class="form-inline my-2 my-lg-0">
                    //     <input class="form-control mr-sm-2" type="text" placeholder="Search" />
                    //     <button class="btn btn-outline-success my-2 my-sm-0" type="submit">{"Search"}</button>
                    // </form>
                </div>
            </nav>
        }
    }
}
