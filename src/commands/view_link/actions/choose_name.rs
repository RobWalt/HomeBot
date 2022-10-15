use crate::app_struct::app_state::app_command::view_link::payload::ViewLinkStatePayload;
use crate::app_struct::app_state::app_command::view_link::ViewLinkState;
use crate::app_struct::app_state::app_command::AppCommand;
use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::payloads::{Payload, ViewLinkPayload};
use crate::types::callback::MyCallbackFormat;
use crate::utils::get_chunk_size;
use anyhow::Result;
use telegram_bot::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn get_category_and_continue_with_name(
    app: &mut App,
    view_link_content: ViewLinkPayload,
) -> Result<()> {
    if let ViewLinkPayload::CategoryChosen(category) = view_link_content {
        if let Some(link_names) = app.link_store.get(&category) {
            let names = link_names.keys().cloned().collect::<Vec<_>>();
            let chunk_size = get_chunk_size(&names);
            let buttons = names
                .chunks(chunk_size)
                .map(|chunk| {
                    chunk
                        .iter()
                        .cloned()
                        .map(|name| {
                            let button_name = name.to_string();
                            let cb = MyCallbackFormat::new(Payload::ViewLink(
                                ViewLinkPayload::NameChosen(name),
                            ));
                            InlineKeyboardButton::callback(button_name, cb.callback_text())
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let mut keyboard = InlineKeyboardMarkup::new();
            for row in buttons {
                keyboard.add_row(row);
            }

            app.update_active_text_and_keyboard("Waehle einen Link aus:", keyboard)
                .await?;
            app.set_state(AppState::ExecutingCommand(AppCommand::ViewLink(
                ViewLinkState::WaitForPayload(ViewLinkStatePayload::WaitingForNameSelection(
                    category,
                )),
            )));
        }
    }
    Ok(())
}
