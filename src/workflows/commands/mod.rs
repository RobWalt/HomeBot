mod general_commands;
mod special_commands;

use teloxide::dptree;

use crate::app::AppState;
use crate::workflows::HandlerType;

pub fn command_handler() -> HandlerType {
    dptree::case![AppState::Neutral(app)]
        .branch(general_commands::create_handler())
        .branch(special_commands::create_handler())
}
