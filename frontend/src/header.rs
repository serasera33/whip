use crate::components::buttons::{button_nav, button_nav2};
use crate::router::{is_user_logged_in, Route};
use zoon::{Also, Element, RawEl, RawHtmlEl, Row, SignalExtBool};

pub fn header() -> impl Element {
    Row::new()
        .item(button_nav("Home", Route::Root))
        .item(based_on_log())
}

pub fn based_on_log() -> impl Element {
    RawHtmlEl::new("div").class("div").child_signal(is_user_logged_in().signal()
        .map_bool(|| button_nav2("Logout", Route::Logout),
                  || button_nav2("Login", Route::Login)))
}