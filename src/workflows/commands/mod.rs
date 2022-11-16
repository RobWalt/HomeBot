mod callbacks;
mod general_commands;
mod special_commands;

use teloxide::dispatching::HandlerExt;
use teloxide::dptree;
use teloxide::types::CallbackQuery;

use crate::app::{AppState, MainDialogue};
use crate::links::{EditLinkStateStorage, EditLinkStateWaitingFor};
use crate::workflows::HandlerType;

use anyhow::Result;

pub use special_commands::CreateLinkStarting;

pub fn command_message_handler() -> HandlerType {
    dptree::entry()
        .branch(
            dptree::case![AppState::Neutral(app)]
                .branch(general_commands::create_handler())
                .branch(special_commands::create_handler()),
        )
        .branch(reset_menu_on_text_answer_handler())
}

pub fn command_callback_handler() -> HandlerType {
    dptree::entry()
        .branch(
            dptree::case![AppState::WaitingForLinkCommand(app)].endpoint(callbacks::link_handler),
        )
        .branch(
            dptree::case![AppState::WaitingForRecipeCommand(app)]
                .endpoint(callbacks::recipe_handler),
        )
        .branch(
            dptree::case![AppState::LinkEditWaitingForCommand(app)]
                .enter_dialogue::<CallbackQuery, EditLinkStateStorage, EditLinkStateWaitingFor>()
                .endpoint(callbacks::edit_link_handler),
        )
}

fn reset_menu_on_text_answer_handler() -> HandlerType {
    dptree::entry()
        .branch(dptree::case![AppState::WaitingForLinkCommand(app)].endpoint(reset_handler))
        .branch(dptree::case![AppState::WaitingForRecipeCommand(app)].endpoint(reset_handler))
}

async fn reset_handler(dialogue: MainDialogue) -> Result<()> {
    log::info!("Executing");
    if let Some(app) = dialogue.get().await?.and_then(|state| state.get_app()) {
        dialogue.update(AppState::Neutral(app)).await?;
    }
    Ok(())
}
