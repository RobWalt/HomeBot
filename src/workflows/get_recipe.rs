use teloxide::prelude::Requester;
use teloxide::types::Message;
use teloxide::Bot;

use crate::app::{AppState, LastMessage, LastMessageDialogue, MainDialogue};

use anyhow::Result;

pub async fn handle_get_recipe_entry_point(
    bot: Bot,
    dialogue: MainDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::WaitingForRecipeCommand(mut app)) = dialogue.get().await? {
        let msg_text = if let Some(recipe) = app.get_random_non_repeating_recipe() {
            format!("Hier ist dein Rezept von heute:\n\n{}", recipe.0)
        } else if let Some(recipe) = app.get_one_of_last_three_recipes() {
            format!("Leider sind noch nicht sehr viele Rezepte gespeichert. Hier ist ein Rezept, dass du eventuell gerade erst bekommen hast:\n\n{}", recipe.0)
        } else {
            "Leider ist noch kein Rezept abgespeichert worden.".to_owned()
        };
        let bot_msg = bot.send_message(msg.chat.id, msg_text).await?;
        last_message
            .update(LastMessage(vec![bot_msg.id, msg.id]))
            .await?;
        dialogue.update(AppState::Neutral(app)).await?;
    }
    Ok(())
}
