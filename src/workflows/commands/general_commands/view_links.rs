use anyhow::Result;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message};
use teloxide::Bot;

use crate::app::{AppState, LastMessage, LastMessageDialogue, MainDialogue};
use crate::links::LinkCategory;
pub async fn enter_dialogue(
    bot: Bot,
    dialogue: MainDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    if let Some(AppState::Neutral(app)) = dialogue.get().await? {
        let categories = app
            .link_storage
            .keys()
            .map(|LinkCategory(category)| InlineKeyboardButton::callback(category, category))
            .collect::<Vec<_>>();
        let bot_msg = if categories.is_empty() {
            let bot_msg: Message = bot
                .send_message(
                    msg.chat.id,
                    "Keine Kategorien vorhanden. Bitte archiviere erst mindestens einen Link",
                )
                .await?;
            dialogue.update(AppState::Neutral(app)).await?;
            bot_msg
        } else {
            let keyboard = InlineKeyboardMarkup::new(vec![categories]);
            let bot_msg: Message = bot
                .send_message(
                    msg.chat.id,
                    "Unter welcher Kategorie ist der Link gespeichert?",
                )
                .reply_markup(keyboard)
                .await?;
            dialogue.update(AppState::LinkViewing(app)).await?;
            bot_msg
        };
        last_message
            .update(LastMessage(vec![bot_msg.id, msg.id]))
            .await?;
    }
    Ok(())
}
