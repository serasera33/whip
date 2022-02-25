use shared::user::*;
use zoon::events::Input;
use zoon::*;
use shared::UpMsg;
use crate::components::buttons::button_apply;
use crate::messenger::send_up_msg;

pub fn page() -> impl Element {
    Column::new()
        .item(
            Row::new().item(Paragraph::new().content("Login")).item(
                TextInput::new()
                    .placeholder(Placeholder::new("Enter your email id"))
                    .input_type(InputType::text())
                    .label_hidden("")
                    .on_change(|val| {
                        let mut lock = attempt_login().lock_mut();
                        let x = val.parse::<String>();
                        match x {
                            Ok(email_id) => lock.email_id = email_id,
                            Err(err) => {}
                        }
                    }),
            ),
        )
        .item(
            Row::new().item(Paragraph::new().content("Password")).item(
                TextInput::new()
                    .placeholder(Placeholder::new("Enter your password"))
                    .input_type(InputType::password())
                    .label_hidden("")
                    .on_change(|val| {
                        let mut lock = attempt_login().lock_mut();
                        let x = val.parse::<String>();
                        match x {
                            Ok(password) => lock.password = password,
                            Err(err) => {}
                        }
                    }),
            ),
        )
        .item(button_apply(|| {
            let mut lock = attempt_login().lock_mut();
            let la = LoginAttempt::from(&lock);
            zoon::println!("Updating {:?}", la);
            send_up_msg(UpMsg::Auth(la));
        }))
}

#[static_ref]
pub fn attempt_login() -> &'static Mutable<LoginAttempt> {
    Mutable::new(LoginAttempt::init())
}
