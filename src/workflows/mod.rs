mod commands;
mod create_links;
mod view_links;

use anyhow::{Error, Result};
use teloxide::dispatching::{DefaultKey, DpHandlerDescription, HandlerExt, UpdateFilterExt};
use teloxide::dptree::Handler;
use teloxide::prelude::{DependencyMap, Dispatcher, Requester};
use teloxide::types::{CallbackQuery, Message, Update};
use teloxide::{dptree, Bot};

use crate::app::{
    AppState, LastMessage, LastMessageDialogue, LastMessageStorage, MainStateStorage,
};
use crate::links::{NewLinkStateStorage, ViewLinkStateStorage};
use crate::persistence::loading::load_on_first_run;
use crate::persistence::saving::save_every_run;

type HandlerType = Handler<'static, DependencyMap, Result<()>, DpHandlerDescription>;
const NEW: &str = "Neu Erstellen";

pub fn create_dispatcher(bot: Bot) -> Dispatcher<Bot, Error, DefaultKey> {
    let handler = dptree::entry()
        .branch(create_message_handler(Update::filter_message()))
        .branch(create_callback_handler(Update::filter_callback_query()));
    let dependencies = dptree::deps![
        MainStateStorage::new(),
        NewLinkStateStorage::new(),
        ViewLinkStateStorage::new(),
        LastMessageStorage::new()
    ];
    Dispatcher::builder(bot, handler)
        .dependencies(dependencies)
        .enable_ctrlc_handler()
        .build()
}

fn create_message_handler(handler: HandlerType) -> HandlerType {
    handler
        .enter_dialogue::<Message, MainStateStorage, AppState>()
        .inspect_async(load_on_first_run)
        .inspect_async(save_every_run)
        .enter_dialogue::<Message, LastMessageStorage, LastMessage>()
        .inspect_async(delete_last_message)
        .branch(commands::command_handler())
        .branch(create_links::message_handler())
        .branch(view_links::message_handler())
}

fn create_callback_handler(handler: HandlerType) -> HandlerType {
    handler
        .enter_dialogue::<CallbackQuery, MainStateStorage, AppState>()
        .inspect_async(load_on_first_run)
        .inspect_async(save_every_run)
        .enter_dialogue::<CallbackQuery, LastMessageStorage, LastMessage>()
        .inspect_async(delete_last_message)
        .branch(create_links::callback_handler())
        .branch(view_links::callback_handler())
}

async fn delete_last_message(bot: Bot, last_message: LastMessageDialogue) {
    if let Some(LastMessage(msg_ids)) = last_message.get().await.ok().flatten() {
        for msg_id in msg_ids.iter() {
            _ = bot.delete_message(last_message.chat_id(), *msg_id).await;
        }
    }
}
