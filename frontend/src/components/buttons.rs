use crate::router::{Route, router};
use std::env::Args;
use zoon::*;
use crate::app::set_page_id;

pub fn button_apply(on_click_cl: impl FnOnce() + Clone + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let hovered_signal2 = hovered.signal();
    Button::new()
        .s(Borders::all_signal(hovered_signal.map_bool(
            || Border::new().color(hsluv!(50, 0, 0, 32)),
            || Border::new().color(hsluv!(0, 0, 0, 0)),
        )))
        .s(Background::new().color_signal({
            hovered_signal2.map_bool(|| named_color::BLUE_0, || named_color::YELLOW_0)
        }))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("Apply")
        .on_click(on_click_cl)
}

pub fn button_nav(lbl: &str, route: Route) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let hovered_signal2 = hovered.signal();
    Link::new()
        .label(lbl)
        .s(Borders::all_signal(hovered_signal.map_bool(
            || Border::new().color(named_color::YELLOW_2),
            || Border::new().color(hsluv!(0, 0, 0, 0)),
        )))
        .s(Background::new().color_signal({
            hovered_signal2.map_bool(|| named_color::BLUE_4, || named_color::BLUE_2)
        }))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .to(route)
}

pub fn button_nav2(lbl: &str, route: Route) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let hovered_signal2 = hovered.signal();
    Button::new()
        .s(Borders::all_signal(hovered_signal.map_bool(
            || Border::new().color(named_color::YELLOW_2),
            || Border::new().color(hsluv!(0, 0, 0, 0)),
        )))
        .s(Background::new().color_signal({
            hovered_signal2.map_bool(|| named_color::BLUE_4, || named_color::BLUE_2)
        }))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label(lbl)
        .on_click(move || {
            router().replace(route);
        })
}