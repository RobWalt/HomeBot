use crate::app_struct::app_state::app_command::archive_link::ArchiveLinkState;
use crate::app_struct::app_state::app_command::main_menu::MainMenuState;
use crate::app_struct::app_state::app_command::view_link::ViewLinkState;
use crate::app_struct::app_state::app_command::AppCommand;
use crate::app_struct::App;
use crate::commands::archive_link::entry_point::{
    handle_archive_link_button_presses, handle_archive_link_texts,
};
use crate::commands::main_menu::entry_point::handle_main_menu_button_presses;
use crate::commands::view_link::entry_points::handle_view_link_button_presses;
use crate::payloads::Payload;
use anyhow::Result;
use telegram_bot::{CallbackQuery, Message, MessageChat, MessageKind};

#[derive(Debug, Clone)]
pub enum MessageOrQuery {
    Message(Message),
    Query(CallbackQuery),
}

pub async fn handle_executing_command(
    app: &mut App,
    message_or_query: MessageOrQuery,
    app_command: AppCommand,
) -> Result<()> {
    match message_or_query {
        MessageOrQuery::Message(message) => {
            handle_executing_command_message(app, message, app_command).await?;
        }
        MessageOrQuery::Query(query) => {
            handle_executing_command_buttons(app, query, app_command).await?;
        }
    }

    Ok(())
}

async fn handle_executing_command_message(
    app: &mut App,
    message: Message,
    app_command: AppCommand,
) -> Result<()> {
    if let MessageChat::Supergroup(g) = message.chat {
        if g.eq(&app.group) {
            if let MessageKind::Text { ref data, .. } = message.kind {
                match app_command {
                    AppCommand::ArchiveLink(ArchiveLinkState::WaitForMessage(
                        archive_link_state,
                    )) => {
                        app.delete_msg(message.id).await?;
                        handle_archive_link_texts(app, data.clone(), archive_link_state).await?;
                    }
                    _ => {
                        // button presses not handled here
                    }
                }
            }
        }
    }

    Ok(())
}

async fn handle_executing_command_buttons(
    app: &mut App,
    query: CallbackQuery,
    app_command: AppCommand,
) -> Result<()> {
    let payload = parse_query(&query);
    match app_command {
        AppCommand::ArchiveLink(ArchiveLinkState::WaitForPayload(archive_link_state)) => {
            if let Some(Payload::ArchiveLink(archive_link_payload)) = payload {
                handle_archive_link_button_presses(
                    app,
                    archive_link_payload,
                    archive_link_state.clone(),
                )
                .await?;
            }
        }
        AppCommand::MainMenu(MainMenuState::WaitForPayload(main_menu_state)) => {
            if let Some(Payload::MainMenu(main_menu_payload)) = payload {
                handle_main_menu_button_presses(app, main_menu_payload, main_menu_state.clone())
                    .await?;
            }
        }
        AppCommand::ViewLink(ViewLinkState::WaitForPayload(view_link_state)) => {
            if let Some(Payload::ViewLink(view_link_payload)) = payload {
                handle_view_link_button_presses(app, view_link_payload, view_link_state.clone())
                    .await?;
            }
        }
        _ => {
            // unhandled since no button presses are awaited
        }
    }

    Ok(())
}

fn parse_query(query: &CallbackQuery) -> Option<Payload> {
    query
        .data
        .as_ref()
        .and_then(|data| serde_json::from_str(data).ok())
}
