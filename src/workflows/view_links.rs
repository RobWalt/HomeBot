use anyhow::Result;
use teloxide::dispatching::HandlerExt;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message};
use teloxide::{dptree, Bot};

use crate::app::{AppState, LastMessage, LastMessageDialogue, MainDialogue};
use crate::links::{
    Link, LinkCategory, LinkName, ViewLinkDialogue, ViewLinkStateStorage, ViewLinkStateWaitingFor,
};

use super::HandlerType;

pub fn message_handler() -> HandlerType {
    dptree::case![AppState::LinkViewing(app)]
        .enter_dialogue::<Message, ViewLinkStateStorage, ViewLinkStateWaitingFor>()
}

pub fn callback_handler() -> HandlerType {
    dptree::case![AppState::LinkViewing(app)]
        .enter_dialogue::<CallbackQuery, ViewLinkStateStorage, ViewLinkStateWaitingFor>()
        .branch(dptree::case![ViewLinkStateWaitingFor::Category].endpoint(save_category_ask_name))
        .branch(
            dptree::case![ViewLinkStateWaitingFor::Name(category)]
                .endpoint(receive_name_return_link),
        )
}

async fn save_category_ask_name(
    bot: Bot,
    cb_query: CallbackQuery,
    dialogue: ViewLinkDialogue,
    main_dialogue: MainDialogue,
    last_message: LastMessageDialogue,
) -> Result<()> {
    if let Some(AppState::LinkViewing(app)) = main_dialogue.get().await? {
        if let Some(msg) = cb_query.message {
            if let Some(data) = cb_query.data {
                let category = LinkCategory(data);
                let names = app
                    .link_storage
                    .get(&category)
                    .iter()
                    .flat_map(|name_map| name_map.keys())
                    .map(|LinkName(name)| InlineKeyboardButton::callback(name, name))
                    .collect::<Vec<_>>();
                let bot_msg = if names.is_empty() {
                    let bot_msg: Message = bot
                        .send_message(msg.chat.id, "Es sind noch keine Links unter dieser Kategorie vorhanden. Bitte archiviere mindestens einen Link")
                        .await?;
                    main_dialogue.update(AppState::Neutral(app)).await?;
                    bot_msg
                } else {
                    let keyboard = InlineKeyboardMarkup::new(vec![names]);
                    dialogue
                        .update(ViewLinkStateWaitingFor::Name(category))
                        .await?;
                    let bot_msg: Message = bot
                        .send_message(msg.chat.id, "Welchen Link moechtest du dir ansehen?")
                        .reply_markup(keyboard)
                        .await?;
                    bot_msg
                };
                last_message.update(LastMessage(vec![bot_msg.id])).await?;
            }
        }
    }
    Ok(())
}

async fn receive_name_return_link(
    bot: Bot,
    cb_query: CallbackQuery,
    dialogue: ViewLinkDialogue,
    main_dialogue: MainDialogue,
) -> Result<()> {
    if let Some(AppState::LinkViewing(app)) = main_dialogue.get().await? {
        if let Some(ViewLinkStateWaitingFor::Name(category)) = dialogue.get().await? {
            if let Some(msg) = cb_query.message {
                if let Some(data) = cb_query.data {
                    let name = LinkName(data);
                    if let Some(Link(link)) = app
                        .link_storage
                        .get(&category)
                        .and_then(|link_map| link_map.get(&name))
                    {
                        bot.send_message(msg.chat.id, format!("Hier ist der Link:\n\n{link}"))
                            .await?;
                        dialogue.exit().await?;
                        main_dialogue.update(AppState::Neutral(app)).await?;
                    }
                }
            }
        }
    }
    Ok(())
}
