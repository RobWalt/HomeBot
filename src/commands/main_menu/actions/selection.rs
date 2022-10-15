use std::time::Duration;

use crate::app_struct::app_state::app_command::view_link::payload::ViewLinkStatePayload;
use crate::app_struct::app_state::app_command::view_link::ViewLinkState;
use crate::app_struct::app_state::app_command::AppCommand;
use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::payloads::{MainMenuPayload, Payload, ViewLinkPayload};
use crate::types::callback::MyCallbackFormat;
use crate::utils::get_chunk_size;
use anyhow::Result;
use telegram_bot::{InlineKeyboardButton, InlineKeyboardMarkup};
use tokio::time::delay_for;

pub async fn main_menu_handler(app: &mut App, main_menu_content: MainMenuPayload) -> Result<()> {
    app.delete_active_msg().await?;
    match main_menu_content {
        MainMenuPayload::Links => handle_view_link_button(app).await?,
        MainMenuPayload::Dismiss => {
            app.send_msg("Tschuess!").await?;
            delay_for(Duration::from_secs(5)).await;
            app.delete_active_msg().await?;
            app.set_state(AppState::WaitingForInputs);
        }
    };
    Ok(())
}

async fn handle_view_link_button(app: &mut App) -> Result<()> {
    if !app.link_store.is_empty() {
        let link_categories = app.link_store.keys().cloned().collect::<Vec<_>>();
        let chunk_size = get_chunk_size(&link_categories);
        let buttons = link_categories
            .chunks(chunk_size)
            .map(|chunk| {
                chunk
                    .iter()
                    .cloned()
                    .map(|category| {
                        let button_name = category.to_string();
                        let cb = MyCallbackFormat::new(Payload::ViewLink(
                            ViewLinkPayload::CategoryChosen(category),
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
        app.set_state(AppState::ExecutingCommand(AppCommand::ViewLink(
            ViewLinkState::WaitForPayload(ViewLinkStatePayload::WaitingForCategorySelection),
        )));
        app.update_active_text_and_keyboard("Waehle eine Kategorie aus!", keyboard)
            .await?;
    } else {
        app.send_msg("Keine Links vorhanden").await?;
        delay_for(Duration::from_secs(5)).await;
        _ = app.delete_active_msg().await;
    }

    Ok(())
}
