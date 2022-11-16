use teloxide::dispatching::HandlerExt;
use teloxide::prelude::Requester;
use teloxide::types::{CallbackQuery, Message};
use teloxide::{dptree, Bot};

use anyhow::Result;

use crate::app::{AppState, LastMessage, LastMessageDialogue, MainDialogue};
use crate::recipe::{AddRecipeDialogue, AddRecipeStateStorage, AddRecipeStateWaitingFor, Recipe};

use super::HandlerType;

pub fn message_handler() -> HandlerType {
    dptree::case![AppState::RecipeStoring(app)]
        .enter_dialogue::<Message, AddRecipeStateStorage, AddRecipeStateWaitingFor>()
        .branch(
            dptree::case![AddRecipeStateWaitingFor::RecipeDescription].endpoint(handle_save_recipe),
        )
}

pub fn callback_handler() -> HandlerType {
    dptree::case![AppState::RecipeStoring(app)]
        .enter_dialogue::<CallbackQuery, AddRecipeStateStorage, AddRecipeStateWaitingFor>()
}

pub async fn handle_add_recipe_entry_point(
    bot: Bot,
    dialogue: MainDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::WaitingForRecipeCommand(app)) = dialogue.get().await? {
        let bot_msg = bot
            .send_message(msg.chat.id, "Bitte sende mir ein Rezept zum abspeichern.")
            .await?;
        dialogue.update(AppState::RecipeStoring(app)).await?;
        last_message
            .update(LastMessage(vec![bot_msg.id, msg.id]))
            .await?;
    }
    Ok(())
}

pub async fn handle_save_recipe(
    bot: Bot,
    dialogue: MainDialogue,
    recipe_dialogue: AddRecipeDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::RecipeStoring(mut app)) = dialogue.get().await? {
        if let Some(data) = msg.text() {
            let bot_msg = bot
                .send_message(msg.chat.id, "Rezept erfolgreich gespeichert!")
                .await?;
            app.add_recipe(Recipe(data.to_owned()));
            recipe_dialogue.exit().await?;
            dialogue.update(AppState::Neutral(app)).await?;
            last_message
                .update(LastMessage(vec![bot_msg.id, msg.id]))
                .await?;
        }
    }
    Ok(())
}
