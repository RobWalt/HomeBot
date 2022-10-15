mod app_struct;
mod commands;
mod payloads;
mod revive;
mod run;
mod serialization;
mod types;
mod utils;

use telegram_bot::Api;
use telegram_bot::CanSendMessage;

use crate::app_struct::App;
use crate::utils::parse_group;
use crate::utils::read_data;
use crate::utils::token_path;

use anyhow::Result;

use self::revive::try_restart_app;
use self::run::run_app;
use self::utils::group_path;

const STARTUP_HELP_MESSAGE: &str = "Hallo, schreibe \"schmot\" um mich zu nutzen!";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let token = read_data(token_path()?)?;
    let group_raw = read_data(group_path()?)?;
    let group = parse_group(group_raw)?;
    let api = Api::new(token.trim());

    api.send(group.clone().text(STARTUP_HELP_MESSAGE)).await?;

    let mut stream = api.stream();
    let mut app = App::new(api, group);

    loop {
        log::info!("A new instance of the bot was started!");
        if let Err(error) = run_app(&mut app, &mut stream).await {
            log::info!("trying to restart ...");
            try_restart_app(&mut app, error).await?;
        }
    }
}
