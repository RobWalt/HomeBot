use std::time::Duration;

use futures::StreamExt;
use telegram_bot::{UpdateKind, UpdatesStream};
use tokio::time::delay_for;

use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::commands::entry_points::executing::{handle_executing_command, MessageOrQuery};
use crate::commands::entry_points::starting::handle_starting_command;

use anyhow::Result;

pub async fn run_app(app: &mut App, stream: &mut UpdatesStream) -> Result<()> {
    while let Some(update) = stream.next().await.transpose()? {
        log::info!("\n\n\nUpdate: {update:?}\n\n\n");
        let old_appstate = app.state.clone();
        let old_links = app.link_store.clone();
        let old_active_msg = app.active_msg;
        while match (update.kind.clone(), app.state.clone()) {
            (UpdateKind::CallbackQuery(query), AppState::ExecutingCommand(app_command)) => {
                let data = MessageOrQuery::Query(query);
                handle_executing_command(app, data.clone(), app_command.clone()).await
            }
            (UpdateKind::Message(message), AppState::ExecutingCommand(app_command)) => {
                let data = MessageOrQuery::Message(message);
                handle_executing_command(app, data, app_command).await
            }
            (UpdateKind::Message(message), AppState::WaitingForInputs) => {
                handle_starting_command(app, message).await
            }

            _ => {
                Ok(())
                // unhandled
            }
        }
        .is_err()
        {
            app.state = old_appstate.clone();
            app.active_msg = old_active_msg;
            app.link_store = old_links.clone();
            delay_for(Duration::from_secs(5)).await;
        }
        app.save()?;
    }
    Ok(())
}
