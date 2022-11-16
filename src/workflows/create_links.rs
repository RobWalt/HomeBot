use anyhow::Result;
use teloxide::dispatching::HandlerExt;
use teloxide::dptree;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, Message};
use teloxide::Bot;

use crate::app::{AppState, LastMessage, LastMessageDialogue, MainDialogue};
use crate::links::{
    LinkCategory, LinkName, NewLinkDialogue, NewLinkStateStorage, NewLinkStateWaitingFor,
};

use super::commands::CreateLinkStarting;
use super::{HandlerType, NEW};

pub fn message_handler() -> HandlerType {
    dptree::case![AppState::LinkStoring(app, link)]
        .enter_dialogue::<Message, NewLinkStateStorage, NewLinkStateWaitingFor>()
        .branch(dptree::case![NewLinkStateWaitingFor::NewCategory].endpoint(create_new_category))
        .branch(
            dptree::case![NewLinkStateWaitingFor::Name(category)].endpoint(create_new_link_name),
        )
}

pub fn callback_handler() -> HandlerType {
    dptree::case![AppState::LinkStoring(app, link)]
        .enter_dialogue::<CallbackQuery, NewLinkStateStorage, NewLinkStateWaitingFor>()
        .branch(dptree::case![NewLinkStateWaitingFor::Entry].endpoint(enter_create_link))
        .branch(dptree::case![NewLinkStateWaitingFor::Category].endpoint(save_category_ask_name))
}

async fn enter_create_link(
    bot: Bot,
    cb_query: CallbackQuery,
    dialogue: NewLinkDialogue,
    main_dialogue: MainDialogue,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::LinkStoring(app, _)) = main_dialogue.get().await? {
        if let Some(msg) = cb_query.message {
            if let Some(data) = cb_query
                .data
                .and_then(|data| CreateLinkStarting::try_from(data.as_str()).ok())
            {
                match data {
                    CreateLinkStarting::Go => {
                        let keyboard = app.category_keyboard();
                        let new_category = vec![InlineKeyboardButton::callback(NEW, NEW)];
                        let keyboard = keyboard.append_row(new_category);
                        let bot_msg: Message = bot
                            .send_message(
                                msg.chat.id,
                                "Unter welcher Kategorie soll der Link abgespeichert werden?",
                            )
                            .reply_markup(keyboard)
                            .await?;
                        dialogue.update(NewLinkStateWaitingFor::Category).await?;
                        last_message
                            .update(LastMessage(vec![bot_msg.id, msg.id]))
                            .await?;
                    }
                    CreateLinkStarting::Dismiss => {
                        main_dialogue.update(AppState::Neutral(app)).await?;
                        last_message.update(LastMessage(vec![msg.id])).await?;
                    }
                }
            }
        }
    }
    Ok(())
}
async fn create_new_category(
    bot: Bot,
    msg: Message,
    dialogue: NewLinkDialogue,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    let data = msg
        .text()
        .unwrap_or_default()
        .chars()
        .take(50)
        .collect::<String>();
    let category = LinkCategory(data);
    dialogue
        .update(NewLinkStateWaitingFor::Name(category))
        .await?;
    let bot_msg: Message = bot
        .send_message(
            msg.chat.id,
            "Unter welchem Namen soll der Link gespeichert werden?",
        )
        .await?;
    last_message
        .update(LastMessage(vec![bot_msg.id, msg.id]))
        .await?;
    Ok(())
}

async fn create_new_link_name(
    bot: Bot,
    msg: Message,
    dialogue: NewLinkDialogue,
    main_dialogue: MainDialogue,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(NewLinkStateWaitingFor::Name(category)) = dialogue.get().await? {
        if let Some(AppState::LinkStoring(mut app, link)) = main_dialogue.get().await? {
            let data = msg
                .text()
                .unwrap_or_default()
                .chars()
                .take(50)
                .collect::<String>();
            let name = LinkName(data);
            app.add_link(category, name, link);
            dialogue.exit().await?;
            main_dialogue.update(AppState::Neutral(app)).await?;
            let bot_msg: Message = bot.send_message(msg.chat.id, "Gespeichert").await?;
            last_message
                .update(LastMessage(vec![bot_msg.id, msg.id]))
                .await?;
        }
    }
    Ok(())
}

async fn save_category_ask_name(
    bot: Bot,
    cb_query: CallbackQuery,
    dialogue: NewLinkDialogue,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(msg) = cb_query.message {
        if let Some(data) = cb_query.data {
            let bot_msg = if data == NEW {
                let bot_msg: Message = bot
                    .send_message(msg.chat.id, "Wie soll die neue Kategorie heissen?")
                    .await?;
                dialogue.update(NewLinkStateWaitingFor::NewCategory).await?;
                bot_msg
            } else {
                let category = LinkCategory(data);
                dialogue
                    .update(NewLinkStateWaitingFor::Name(category))
                    .await?;
                let bot_msg: Message = bot
                    .send_message(
                        msg.chat.id,
                        "Unter welchem Namen soll der Link gespeichert werden?",
                    )
                    .await?;
                bot_msg
            };
            last_message.update(LastMessage(vec![bot_msg.id])).await?;
        }
    }
    Ok(())
}
