use zoon::*;
use shared::{Message, UpMsg};
use crate::components::buttons::button_apply;
use crate::messenger::send_up_msg;

pub fn page() -> impl Element {

    Row::new().item(button_apply(|| {
        send_up_msg(UpMsg::Log(Message::new("Dummy")));
    }))
}
