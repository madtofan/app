use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    services::requests::{get_token, request_get, set_token},
    types::{
        auth::{UserInfo, UserInfoWrapper},
        error::Error,
    },
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(AppContext)]
pub fn app_context(props: &Props) -> Html {
    let app_context = use_state(UserInfo::default);
    let current_user =
        use_async(async move { request_get::<UserInfoWrapper>("/user".to_string()).await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    {
        let app_context = app_context.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    app_context.set(user_info.user.clone());
                }

                if let Some(error) = &current_user.error {
                    match error {
                        Error::Unauthorized | Error::Forbidden => set_token(None),
                        _ => (),
                    }
                }
                || ()
            },
            current_user,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={app_context}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
