use anyhow::Result;
use strum::{Display, IntoEnumIterator};
use strum::{EnumIter, EnumString};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message};
use teloxide::{dptree, Bot};

use crate::app::{AppState, LastMessage, LastMessageDialogue, MainDialogue};
use crate::links::{Link, RawLinkContainingMessage};
use crate::workflows::HandlerType;

pub fn command_handler() -> HandlerType {
    dptree::filter(noticed_link).endpoint(ask_create_link)
}

fn noticed_link(msg: Message) -> bool {
    let link_starts = ["https", "http", "www"];
    msg.text()
        .map(|text| link_starts.iter().any(|start| text.contains(start)))
        .unwrap_or(false)
}

#[derive(EnumIter, Display, EnumString)]
pub enum CreateLinkStarting {
    #[strum(serialize = "Ja")]
    Go,
    #[strum(serialize = "Nein")]
    Dismiss,
}

async fn ask_create_link(
    bot: Bot,
    dialogue: MainDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::Neutral(app)) = dialogue.get().await? {
        let buttons = CreateLinkStarting::iter()
            .map(|option| option.to_string())
            .map(|option| {
                vec![InlineKeyboardButton::callback(
                    option.as_str(),
                    option.as_str(),
                )]
            })
            .collect::<Vec<_>>();
        let keyboard = InlineKeyboardMarkup::new(buttons);
        let bot_msg: Message = bot
            .send_message(msg.chat.id, "Soll ich den Link abspeichern?")
            .reply_markup(keyboard)
            .await?;
        let raw_link = RawLinkContainingMessage(msg.text().unwrap_or_default().to_string());
        let link = clean_link(raw_link);
        dialogue.update(AppState::LinkStoring(app, link)).await?;
        last_message.update(LastMessage(vec![bot_msg.id])).await?;
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
