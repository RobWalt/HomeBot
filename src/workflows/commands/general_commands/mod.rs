mod links;
mod recipes;

use anyhow::Result;
use teloxide::dispatching::HandlerExt;
use teloxide::prelude::Requester;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;
use teloxide::{dptree, Bot};

use crate::app::{LastMessageDialogue, MainDialogue};
use crate::workflows::HandlerType;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Verfuegbare Kommandos")]
pub enum StartingCommands {
    #[command(description = "Zeige diese Nachricht an")]
    Help,
    #[command(description = "Suche oder bearbeite gespeicherte Links")]
    Links,
    #[command(description = "Finde oder speichere Rezepte")]
    Recipe,
}

pub fn create_handler() -> HandlerType {
    dptree::entry()
        .filter_command::<StartingCommands>()
        .endpoint(handle_starting_commands)
}

async fn handle_starting_commands(
    bot: Bot,
    dialogue: MainDialogue,
    msg: Message,
    cmd: StartingCommands,
    last_message: LastMessageDialogue,
) -> Result<()> {
    log::info!("Executing");
    match cmd {
        StartingCommands::Help => execute_help(bot, msg).await,
        StartingCommands::Links => links::enter_dialogue(bot, dialogue, msg, last_message).await,
        StartingCommands::Recipe => recipes::enter_dialogue(bot, dialogue, msg, last_message).await,
    }
}

async fn execute_help(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, format!("{}", StartingCommands::descriptions()))
        .await?;
    Ok(())
}
