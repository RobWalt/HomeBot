use anyhow::Result;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message};
use teloxide::{dptree, Bot};

use crate::app::{AppState, LastMessage, LastMessageDialogue, MainDialogue};
use crate::links::{Link, LinkCategory, RawLinkContainingMessage};
use crate::workflows::{HandlerType, NEW};

pub fn command_handler() -> HandlerType {
    dptree::filter(noticed_link).endpoint(enter_create_link)
}

fn noticed_link(msg: Message) -> bool {
    let link_starts = ["https", "http", "www"];
    msg.text()
        .map(|text| link_starts.iter().any(|start| text.contains(start)))
        .unwrap_or(false)
}

async fn enter_create_link(
    bot: Bot,
    dialogue: MainDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    if let Some(AppState::Neutral(app)) = dialogue.get().await? {
        let existing_categories = app
            .link_storage
            .keys()
            .map(|LinkCategory(category)| {
                InlineKeyboardButton::callback(category.to_owned(), category.to_owned())
            })
            .collect::<Vec<_>>();
        let new_category = vec![InlineKeyboardButton::callback(NEW, NEW)];
        let keyboard = InlineKeyboardMarkup::new(vec![existing_categories, new_category]);
        let bot_msg: Message = bot
            .send_message(
                msg.chat.id,
                "Unter welcher Kategorie soll der Link abgespeichert werden?",
            )
            .reply_markup(keyboard)
            .await?;
        let raw_link = RawLinkContainingMessage(msg.text().unwrap_or_default().to_string());
        let link = clean_link(raw_link);
        dialogue.update(AppState::LinkStoring(app, link)).await?;
        last_message
            .update(LastMessage(vec![bot_msg.id, msg.id]))
            .await?;
    }
    Ok(())
}

fn clean_link(raw_link: RawLinkContainingMessage) -> Link {
    let link_starts = ["https", "http", "www"];
    let RawLinkContainingMessage(raw_link) = raw_link;
    let link = raw_link
        .split(char::is_whitespace)
        .find(|word| link_starts.iter().any(|start| word.contains(start)))
        .unwrap_or_default();
    Link(link.to_owned())
}
