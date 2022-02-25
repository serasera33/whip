use crate::{app, router};
use shared::*;
use zoon::once_cell::race::OnceBox;
use zoon::{once_cell, static_ref, Connection, Mutable, MutableVec, Task};
use crate::app::{PageId, set_page_id};
use crate::router::{is_user_logged_in, Route, router};

fn username() -> &'static Mutable<String> {
    static INSTANCE: OnceBox<Mutable<String>> = OnceBox::new();
    INSTANCE.get_or_init(move || Box::new(Mutable::new("One".to_owned())))
}

fn messages() -> &'static MutableVec<Message> {
    static INSTANCE: OnceBox<MutableVec<Message>> = OnceBox::new();
    INSTANCE.get_or_init(move || Box::new(MutableVec::new()))
}

fn new_message_text() -> &'static Mutable<String> {
    static INSTANCE: OnceBox<Mutable<String>> = OnceBox::new();
    INSTANCE.get_or_init(move || Box::new(Mutable::new(String::new())))
}

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, cor_id| {
        println!("DownMsg received: {:?}", down_msg);

        match down_msg {
            DownMsg::Log(message) => {
                zoon::println!("Received {}", message.text);
            }
            DownMsg::AuthSuccess(authkey) => {
                app::auth_token().set(authkey);
                router::is_user_logged_in().set(true);
                zoon::println!("Successful Authentication {:?}", *is_user_logged_in().lock_mut());
                zoon::println!("Recvd authkey {:?}", app::get_auth_token());
                set_page_id(PageId::Root);
                router().replace(Route::Root);
            },
            DownMsg::AuthDenied => {
                zoon::println!("Authentication Denied");
            }
        }
    })
    .auth_token_getter(|| app::get_auth_token())
}

#[static_ref]
fn received_messages_viewport_y() -> &'static Mutable<i32> {
    Mutable::new(0)
}

pub fn send_up_msg(up_msg: UpMsg) {
    // let m = UpMsg::Log(Message { text: msg });
    Task::start(async {
        let result = connection().send_up_msg(up_msg).await;
        if let Err(error) = result {
            zoon::eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn jump_to_bottom() {
    received_messages_viewport_y().set(i32::MAX);
}
