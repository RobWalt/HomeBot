use anyhow::Result;
use teloxide::types::{CallbackQuery, Message};
use teloxide::Bot;

use crate::app::{
    AppState, EditLinkCommads, LastMessage, LastMessageDialogue, LinkCommands, MainDialogue,
    RecipeCommands,
};
use crate::links::EditLinkDialogue;
use crate::workflows::add_recipe::handle_add_recipe_entry_point;
use crate::workflows::edit_links::{
    enter_edit_link_category, enter_handle_edit_link_name, handle_edit_link_entry_point,
};
use crate::workflows::get_recipe::handle_get_recipe_entry_point;
use crate::workflows::view_links::handle_view_link_entry_point;

pub async fn link_handler(
    bot: Bot,
    dialogue: MainDialogue,
    cb_query: CallbackQuery,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(cmd) = cb_query
        .data
        .and_then(|data| LinkCommands::try_from(data.as_str()).ok())
    {
        if let Some(msg) = cb_query.message {
            match cmd {
                LinkCommands::View => {
                    handle_view_link_entry_point(bot, dialogue, msg, last_message).await?
                }
                LinkCommands::Dismiss => {
                    handle_dismiss_commands(dialogue, last_message, msg).await?
                }
                LinkCommands::Edit => {
                    handle_edit_link_entry_point(bot, dialogue, msg, last_message).await?
                }
            }
        }
    }
    Ok(())
}

pub async fn recipe_handler(
    bot: Bot,
    dialogue: MainDialogue,
    cb_query: CallbackQuery,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(cmd) = cb_query
        .data
        .and_then(|data| RecipeCommands::try_from(data.as_str()).ok())
    {
        if let Some(msg) = cb_query.message {
            match cmd {
                RecipeCommands::GetRandom => {
                    handle_get_recipe_entry_point(bot, dialogue, msg, last_message).await?;
                }
                RecipeCommands::Add => {
                    handle_add_recipe_entry_point(bot, dialogue, msg, last_message).await?;
                }
                RecipeCommands::Dismiss => {
                    handle_dismiss_commands(dialogue, last_message, msg).await?;
                }
            }
        }
    }
    Ok(())
}

pub async fn edit_link_handler(
    bot: Bot,
    dialogue: MainDialogue,
    edit_link_dialogue: EditLinkDialogue,
    cb_query: CallbackQuery,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(cmd) = cb_query
        .data
        .and_then(|data| EditLinkCommads::try_from(data.as_str()).ok())
    {
        if let Some(msg) = cb_query.message {
            match cmd {
                EditLinkCommads::Dismiss => {
                    handle_dismiss_commands(dialogue, last_message, msg).await?
                }
                EditLinkCommads::EditCategory => {
                    enter_edit_link_category(bot, dialogue, edit_link_dialogue, msg, last_message)
                        .await?
                }
                EditLinkCommads::EditName => {
                    enter_handle_edit_link_name(
                        bot,
                        dialogue,
                        edit_link_dialogue,
                        msg,
                        last_message,
                    )
                    .await?
                }
            }
        }
    }
    Ok(())
}

pub async fn handle_dismiss_commands(
    dialogue: MainDialogue,
    last_message: LastMessageDialogue,
    msg: Message,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::WaitingForLinkCommand(app)) = dialogue.get().await? {
        dialogue.update(AppState::Neutral(app)).await?;
        last_message.update(LastMessage(vec![msg.id])).await?;
    }
    Ok(())
}
