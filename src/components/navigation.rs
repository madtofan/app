use yew::prelude::*;

use super::dropdown::Dropdown;
use yew::{function_component, html, use_state, Callback, Properties};
use yew_hooks::use_location;

#[derive(Clone, PartialEq, Properties)]
pub struct NavigationProps {}

#[function_component(Navigation)]
pub fn dropdown(_props: &NavigationProps) -> Html {
    const ACTIVE_CLASS: &str = "block px-4 py-2 mt-2 text-sm font-semibold text-gray-900 bg-gray-200 rounded-lg dark-mode:bg-gray-700 dark-mode:hover:bg-gray-600 dark-mode:focus:bg-gray-600 dark-mode:focus:text-white dark-mode:hover:text-white dark-mode:text-gray-200 hover:text-gray-900 focus:text-gray-900 hover:bg-gray-200 focus:bg-gray-200 focus:outline-none focus:shadow-outline";
    const INACTIVE_CLASS: &str = "block px-4 py-2 mt-2 text-sm font-semibold text-gray-900 bg-transparent rounded-lg dark-mode:bg-transparent dark-mode:hover:bg-gray-600 dark-mode:focus:bg-gray-600 dark-mode:focus:text-white dark-mode:hover:text-white dark-mode:text-gray-200 hover:text-gray-900 focus:text-gray-900 hover:bg-gray-200 focus:bg-gray-200 focus:outline-none focus:shadow-outline";

    let is_open = use_state(|| false);
    let location = use_location();
    let pathname = &location.pathname;

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class="md:flex flex-col md:flex-row md:min-h-screen w-full">
          <div class="flex flex-col w-full md:w-64 text-gray-700 bg-white dark-mode:text-gray-200 dark-mode:bg-gray-800 flex-shrink-0" x-data="{ open: false }">
            <div class="flex-shrink-0 px-8 py-4 flex flex-row items-center justify-between">
              <a href="#" class="text-lg font-semibold tracking-widest text-gray-900 uppercase rounded-lg dark-mode:text-white focus:outline-none focus:shadow-outline">{"Navigation"}</a>
              <button class="md:hidden rounded-lg focus:outline-none focus:shadow-outline" {onclick}>
                <svg fill="currentColor" viewBox="0 0 20 20" class="w-6 h-6">
                  {
                    match *is_open {
                      true => html!(<path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>),
                      false => html!(<path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM9 15a1 1 0 011-1h6a1 1 0 110 2h-6a1 1 0 01-1-1z" clip-rule="evenodd"></path>)
                    }
                  }
                </svg>
              </button>
            </div>
            <nav class={format!("{} flex-grow md:block px-4 pb-4 md:pb-0 md:overflow-y-auto ", match *is_open {
                true => "",
                false => "hidden"
              })
            }>
              <a class={match pathname.as_str() {
                "/" => ACTIVE_CLASS,
                _ => INACTIVE_CLASS
              }} href="/">{ "Profile" }</a>
              <a class={match pathname.as_str() {
                "/projects" => ACTIVE_CLASS,
                _ => INACTIVE_CLASS
              }} href="/projects">{ "Projects" }</a>
              <a class={match pathname.as_str() {
                "/contact" => ACTIVE_CLASS,
                _ => INACTIVE_CLASS
              }} href="/contact">{ "Contact" }</a>
              <Dropdown>
              <a class={match pathname.as_str() {
                "/404" => ACTIVE_CLASS,
                _ => INACTIVE_CLASS
              }} href="#">{ "Link #1" }</a>
              <a class={match pathname.as_str() {
                "/404" => ACTIVE_CLASS,
                _ => INACTIVE_CLASS
              }} href="#">{ "Link #2" }</a>
              <a class={match pathname.as_str() {
                "/404" => ACTIVE_CLASS,
                _ => INACTIVE_CLASS
              }} href="#">{ "Link #3" }</a>
              </Dropdown>
            </nav>
          </div>
        </div>
    }
}
