use zoon::{route, static_ref, Router, *};

use app::*;

use crate::app;

#[route]
#[derive(Debug, Clone, Copy)]
pub enum Route {
    #[route()]
    Root,
    #[route("login")]
    Login,
    #[route("logout")]
    Logout,
}

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::Root) => {
            zoon::println!("Root route requested");
            if !(*is_user_logged_in().lock_mut()) {
                set_page_id(PageId::Login);
                zoon::println!("redirecting to login page");
            } else {
                set_page_id(PageId::Root)
            }
        }
        Some(Route::Login) => {
            zoon::println!("Login route requested");
            if !(*is_user_logged_in().lock_mut()) {
                set_page_id(PageId::Login);
                zoon::println!("redirecting to login page");
            } else {
                router().replace(Route::Root);
                set_page_id(PageId::Root)
            }
        }
        Some(Route::Logout) => {
            zoon::println!("logout route requested");
            is_user_logged_in().set(false);
            router().replace(Route::Login);
            set_page_id(PageId::Login);
        }
        None => {
            zoon::println!("None route requested");
            if !(*is_user_logged_in().lock_mut()) {
                set_page_id(PageId::Login);
                zoon::println!("redirecting to login page");
            } else {
                set_page_id(PageId::Root)
            }
        }
    })
}

#[static_ref]
pub fn is_user_logged_in() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
pub fn live_user() -> &'static Mutable<Option<String>> { Mutable::new(None)}
