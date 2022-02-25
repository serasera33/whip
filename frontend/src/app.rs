use zoon::*;
use crate::{header, home, login};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Root,
    Login
}

#[static_ref]
pub fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Root)
}

#[static_ref]
pub fn auth_token() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

pub fn get_auth_token() -> Option<AuthToken> {
    let mut lock = auth_token().lock_mut();
    let auth = String::from(&*lock);
    Some(AuthToken::new(auth))
}

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

pub fn root() -> impl Element {
    Column::new()
        .s(Width::fill())
        .item(header::header())
        .item(page())
}

fn page() -> impl Element {
    RawHtmlEl::new("div").class("nav").child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Root => { home::page().into_raw_element() }
        PageId::Login => { login::page().into_raw_element() }
    }))
}