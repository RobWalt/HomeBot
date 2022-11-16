use anyhow::Result;
use teloxide::dispatching::HandlerExt;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{CallbackQuery, Message};
use teloxide::{dptree, Bot};

use crate::app::{AppState, EditLinkCommads, LastMessage, LastMessageDialogue, MainDialogue};
use crate::links::{
    EditLinkDialogue, EditLinkStateStorage, EditLinkStateWaitingFor, LinkCategory, LinkName,
};
use crate::utils::create_command_keyboard;

use super::HandlerType;

pub fn message_handler() -> HandlerType {
    dptree::case![AppState::LinkEditing(app)]
        .enter_dialogue::<Message, EditLinkStateStorage, EditLinkStateWaitingFor>()
        .branch(
            dptree::case![EditLinkStateWaitingFor::CategoryNewName(category)]
                .endpoint(handle_edit_category_finish),
        )
        .branch(
            dptree::case![EditLinkStateWaitingFor::NameNewName(category, name)]
                .endpoint(handle_edit_name_finish),
        )
}

pub fn callback_handler() -> HandlerType {
    dptree::case![AppState::LinkEditing(app)]
        .enter_dialogue::<CallbackQuery, EditLinkStateStorage, EditLinkStateWaitingFor>()
        .branch(
            dptree::case![EditLinkStateWaitingFor::Category]
                .endpoint(handle_edit_category_pick_new_name),
        )
        .branch(
            dptree::case![EditLinkStateWaitingFor::NameCategory]
                .endpoint(handle_edit_name_choose_name_in_category),
        )
        .branch(
            dptree::case![EditLinkStateWaitingFor::Name(category)]
                .endpoint(handle_edit_name_pick_new_name),
        )
}

pub async fn handle_edit_link_entry_point(
    bot: Bot,
    dialogue: MainDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::WaitingForLinkCommand(app)) = dialogue.get().await? {
        let keyboard = create_command_keyboard::<EditLinkCommads>();
        let bot_msg: Message = bot
            .send_message(msg.chat.id, "Was willst du bearbeiten?")
            .reply_markup(keyboard)
            .await?;
        dialogue
            .update(AppState::LinkEditWaitingForCommand(app))
            .await?;
        last_message
            .update(LastMessage(vec![bot_msg.id, msg.id]))
            .await?;
    }
    Ok(())
}

pub async fn enter_edit_link_category(
    bot: Bot,
    dialogue: MainDialogue,
    edit_link_dialogue: EditLinkDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::LinkEditWaitingForCommand(app)) = dialogue.get().await? {
        if let Some(EditLinkStateWaitingFor::ChoosePath) = edit_link_dialogue.get().await? {
            let keyboard = app.category_keyboard();
            let bot_msg = bot
                .send_message(msg.chat.id, "Welche Kategorie magst du bearbeiten?")
                .reply_markup(keyboard)
                .await?;

            dialogue.update(AppState::LinkEditing(app)).await?;
            edit_link_dialogue
                .update(EditLinkStateWaitingFor::Category)
                .await?;
            last_message
                .update(LastMessage(vec![msg.id, bot_msg.id]))
                .await?;
        }
    }
    Ok(())
}

pub async fn handle_edit_category_pick_new_name(
    bot: Bot,
    edit_link_dialogue: EditLinkDialogue,
    cb_query: CallbackQuery,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(EditLinkStateWaitingFor::Category) = edit_link_dialogue.get().await? {
        if let Some(msg) = cb_query.message {
            if let Some(data) = cb_query.data {
                let category = LinkCategory(data);
                let bot_msg = bot
                    .send_message(msg.chat.id, "Welchen neuen Namen soll die Kategorie haben?")
                    .await?;
                edit_link_dialogue
                    .update(EditLinkStateWaitingFor::CategoryNewName(category))
                    .await?;
                last_message
                    .update(LastMessage(vec![msg.id, bot_msg.id]))
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn handle_edit_category_finish(
    bot: Bot,
    dialogue: MainDialogue,
    edit_link_dialogue: EditLinkDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::LinkEditing(mut app)) = dialogue.get().await? {
        if let Some(EditLinkStateWaitingFor::CategoryNewName(category)) =
            edit_link_dialogue.get().await?
        {
            if let Some(old_category) = app.link_storage.remove(&category) {
                let new_category_name = LinkCategory(msg.text().unwrap_or_default().to_string());
                app.link_storage.insert(new_category_name, old_category);

                let bot_msg = bot.send_message(msg.chat.id, "Gespeichert").await?;
                edit_link_dialogue.exit().await?;
                dialogue.update(AppState::Neutral(app)).await?;
                last_message
                    .update(LastMessage(vec![msg.id, bot_msg.id]))
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn enter_handle_edit_link_name(
    bot: Bot,
    dialogue: MainDialogue,
    edit_link_dialogue: EditLinkDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::LinkEditWaitingForCommand(app)) = dialogue.get().await? {
        if let Some(EditLinkStateWaitingFor::ChoosePath) = edit_link_dialogue.get().await? {
            let keyboard = app.category_keyboard();
            let bot_msg = bot
                .send_message(
                    msg.chat.id,
                    "In welcher Kategorie magst du einen Namen bearbeiten?",
                )
                .reply_markup(keyboard)
                .await?;

            dialogue.update(AppState::LinkEditing(app)).await?;
            edit_link_dialogue
                .update(EditLinkStateWaitingFor::NameCategory)
                .await?;
            last_message
                .update(LastMessage(vec![msg.id, bot_msg.id]))
                .await?;
        }
    }
    Ok(())
}

pub async fn handle_edit_name_choose_name_in_category(
    bot: Bot,
    dialogue: MainDialogue,
    edit_link_dialogue: EditLinkDialogue,
    cb_query: CallbackQuery,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::LinkEditing(app)) = dialogue.get().await? {
        if let Some(EditLinkStateWaitingFor::NameCategory) = edit_link_dialogue.get().await? {
            if let Some(msg) = cb_query.message {
                if let Some(data) = cb_query.data {
                    let category = LinkCategory(data);
                    let keyboard = app.name_keyboard_for(&category);
                    let bot_msg = bot
                        .send_message(msg.chat.id, "Welchen Namen magst du bearbeiten?")
                        .reply_markup(keyboard)
                        .await?;

                    edit_link_dialogue
                        .update(EditLinkStateWaitingFor::Name(category))
                        .await?;
                    last_message
                        .update(LastMessage(vec![msg.id, bot_msg.id]))
                        .await?;
                }
            }
        }
    }
    Ok(())
}

pub async fn handle_edit_name_pick_new_name(
    bot: Bot,
    edit_link_dialogue: EditLinkDialogue,
    cb_query: CallbackQuery,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(EditLinkStateWaitingFor::Name(category)) = edit_link_dialogue.get().await? {
        if let Some(msg) = cb_query.message {
            if let Some(data) = cb_query.data {
                let name = LinkName(data);
                let bot_msg = bot
                    .send_message(msg.chat.id, "Welchen neuen Namen soll der Link haben?")
                    .await?;

                edit_link_dialogue
                    .update(EditLinkStateWaitingFor::NameNewName(category, name))
                    .await?;
                last_message
                    .update(LastMessage(vec![msg.id, bot_msg.id]))
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn handle_edit_name_finish(
    bot: Bot,
    dialogue: MainDialogue,
    edit_link_dialogue: EditLinkDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::LinkEditing(mut app)) = dialogue.get().await? {
        if let Some(EditLinkStateWaitingFor::NameNewName(category, name)) =
            edit_link_dialogue.get().await?
        {
            if let Some(link) = app
                .link_storage
                .get_mut(&category)
                .and_then(|map| map.remove(&name))
            {
                let new_category_name = LinkName(msg.text().unwrap_or_default().to_string());
                app.link_storage
                    .get_mut(&category)
                    .and_then(|map| map.insert(new_category_name, link));

                let bot_msg = bot.send_message(msg.chat.id, "Gespeichert").await?;
                edit_link_dialogue.exit().await?;
                dialogue.update(AppState::Neutral(app)).await?;
                last_message
                    .update(LastMessage(vec![msg.id, bot_msg.id]))
                    .await?;
            }
        }
    }
    Ok(())
}
