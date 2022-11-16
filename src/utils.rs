use std::fmt::Display;

use strum::IntoEnumIterator;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn create_command_keyboard<T: IntoEnumIterator + Display>() -> InlineKeyboardMarkup {
    let commands = T::iter()
        .map(|command| command.to_string())
        .map(|command| {
            vec![InlineKeyboardButton::callback(
                command.as_str(),
                command.as_str(),
            )]
        })
        .collect::<Vec<_>>();
    InlineKeyboardMarkup::new(commands)
}
