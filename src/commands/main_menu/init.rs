use crate::app_struct::app_state::app_command::main_menu::payload::MainMenuStatePayload;
use crate::app_struct::app_state::app_command::main_menu::MainMenuState;
use crate::app_struct::app_state::app_command::AppCommand;
use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::payloads::{MainMenuPayload, Payload};
use crate::types::callback::MyCallbackFormat;
use anyhow::Result;
use telegram_bot::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn init_menu(app: &mut App) -> Result<()> {
    app.set_state(AppState::ExecutingCommand(AppCommand::MainMenu(
        MainMenuState::WaitForPayload(MainMenuStatePayload::WaitingForButtonPress),
    )));
    let mk_button = |display_text, payload| {
        let cb = MyCallbackFormat::new(payload);
        InlineKeyboardButton::callback(display_text, cb.callback_text())
    };

    let links_button = mk_button(
        "Gespeicherte Links",
        Payload::MainMenu(MainMenuPayload::Links),
    );
    let dismiss_button = mk_button("Schliessen", Payload::MainMenu(MainMenuPayload::Dismiss));

    let mut keyboard = InlineKeyboardMarkup::new();
    keyboard.add_row(vec![links_button, dismiss_button]);

    app.send_msg_with_keyboard("Menu", keyboard).await?;

    Ok(())
}
