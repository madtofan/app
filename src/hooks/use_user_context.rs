use crate::{
    services::{requests::set_token, router::Route},
    types::auth::UserInfo,
};
use core::fmt;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct UseUserContextHandle {
    inner: UseStateHandle<UserInfo>,
    navigator: Navigator,
}

impl UseUserContextHandle {
    pub fn login(&self, value: UserInfo) {
        // Set global token after logged in
        set_token(Some(value.token.clone()));
        self.inner.set(value);
        // Redirect to home page
        self.navigator.push(&Route::Profile);
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        set_token(None);
        self.inner.set(UserInfo::default());
        // Redirect to home page
        self.navigator.push(&Route::Profile);
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            navigator: self.navigator.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
