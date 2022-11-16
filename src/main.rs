mod app;
mod links;
mod persistence;
mod recipe;
mod utils;
mod workflows;

use teloxide::prelude::*;

use crate::workflows::create_dispatcher;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot ...");
    let bot = Bot::from_env();
    log::info!("Bot successful started ...");
    create_dispatcher(bot).dispatch().await;
}
