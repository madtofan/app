use crate::{
    components::templates::center_template::CenterTemplate,
    hooks::use_user_context::use_user_context,
    services::{requests::request_post, router::Route},
    types::auth::UserInfoWrapper,
};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let user_ctx = use_user_context();
    let register_info = use_state(RegisterInfo::default);
    let user_register = {
        let register_info = register_info.clone();
        use_async(async move {
            let request = RegisterInfoWrapper {
                user: (*register_info).clone(),
            };
            request_post::<RegisterInfoWrapper, UserInfoWrapper>("/user".to_string(), request).await
        })
    };

    {
        use_effect_with_deps(
            move |user_register| {
                if let Some(user_info) = &user_register.data {
                    user_ctx.login(user_info.user.clone());
                }
                || ()
            },
            user_register.clone(),
        );
    }

    let onsubmit = {
        let user_register = user_register.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            user_register.run();
        })
    };

    let oninput_username = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info);
        })
    };

    let oninput_email = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info);
        })
    };

    let oninput_password = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info);
        })
    };

    html! {
        <CenterTemplate>
          <a href="/" class="flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white">
              {"Portfolio Rust"}
          </a>
          <div class="w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700">
              <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
                  <h1 class="text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white">
                    {"Sign up a new account"}
                  </h1>
                  <form class="space-y-4 md:space-y-6" {onsubmit}>
                      <div>
                          <label for="username" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Username"}</label>
                          <input type="text" value={register_info.username.clone()} oninput={oninput_username} name="username" id="username" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Username" required={true}/>
                      </div>
                      <div>
                          <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Your email"}</label>
                          <input type="email" value={register_info.email.clone()} oninput={oninput_email} name="email" id="email" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Email" required={true}/>
                      </div>
                      <div>
                          <label for="confirm-password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Password"}</label>
                          <input type="password" value={register_info.password.clone()} oninput={oninput_password} name="password" id="password" placeholder="????????????????????????" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required={true}/>
                      </div>
                      <button type="submit" class="w-full text-white bg-primary-600 hover:bg-primary-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800">{"Sign up"}</button>
                      <Link<Route> to={Route::Login}>
                          {"Login to existing account"}
                      </Link<Route>>
                  </form>
              </div>
          </div>
        </CenterTemplate>
    }
}
