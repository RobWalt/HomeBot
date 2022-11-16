use anyhow::Result;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::Message;
use teloxide::Bot;

use crate::app::{AppState, LastMessage, LastMessageDialogue, LinkCommands, MainDialogue};
use crate::utils::create_command_keyboard;

pub async fn enter_dialogue(
    bot: Bot,
    dialogue: MainDialogue,
    msg: Message,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    if let Some(AppState::Neutral(app)) = dialogue.get().await? {
        let keyboard = create_command_keyboard::<LinkCommands>();
        let bot_msg: Message = bot
            .send_message(msg.chat.id, "Welches Link Kommando willst du nutzen?")
            .reply_markup(keyboard)
            .await?;
        dialogue
            .update(AppState::WaitingForLinkCommand(app))
            .await?;
        last_message
            .update(LastMessage(vec![bot_msg.id, msg.id]))
            .await?;
    }
    Ok(())
}
