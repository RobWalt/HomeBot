use crate::app_struct::app_state::app_command::archive_link::message::ArchiveLinkStateMessage;
use crate::app_struct::app_state::app_command::archive_link::ArchiveLinkState;
use crate::app_struct::app_state::app_command::AppCommand;
use crate::app_struct::app_state::AppState;
use crate::commands::archive_link::entry_point::handle_archive_link_texts;
use crate::commands::main_menu::init::init_menu;
use crate::commands::send_help::send_help;
use crate::types::general::Link;

use telegram_bot::{Message, MessageChat, MessageKind};

use crate::app_struct::App;
use anyhow::Result;

pub async fn handle_starting_command(app: &mut App, message: Message) -> Result<()> {
    match message.chat.clone() {
        MessageChat::Supergroup(g) if g.eq(&app.group) => {
            app.delete_msg(message.id).await?;
            match message.kind {
                MessageKind::Text { ref data, .. } if is_schmot_command(data) => {
                    init_menu(app).await?;
                }
                MessageKind::Text { ref data, .. } if is_help_command(data) => {
                    send_help(app).await?;
                }
                MessageKind::Text { ref data, .. } if is_archive_link_command(data) => {
                    if let Some(link) = extract_link(data) {
                        let new_state = ArchiveLinkStateMessage::StartArchiveLink(link);
                        app.set_state(AppState::ExecutingCommand(AppCommand::ArchiveLink(
                            ArchiveLinkState::WaitForMessage(new_state.clone()),
                        )));
                        handle_archive_link_texts(app, data.clone(), new_state).await?;
                    }
                }
                _ => {
                    // unhandled
                }
            }
        }
        _ => {
            // unhandled
        }
    }
    Ok(())
}

fn is_schmot_command(data: &str) -> bool {
    data == "schmot"
}

fn is_help_command(data: &str) -> bool {
    data == "/help"
}

fn is_archive_link_command(data: &str) -> bool {
    const LINK_STARTS: [&str; 2] = ["https://", "www"];
    LINK_STARTS.iter().any(|start| data.contains(start))
}

fn extract_link(data: &str) -> Option<Link> {
    const LINK_STARTS: [&str; 2] = ["https://", "www"];
    LINK_STARTS
        .iter()
        .find(|start| data.contains(**start))
        .and_then(|link_start| {
            data.split_once(link_start)
                .map(|(_, contains_link_rest)| (link_start, contains_link_rest))
        })
        .and_then(|(link_start, contains_link_rest)| {
            contains_link_rest
                .split_whitespace()
                .next()
                .map(|link_end| link_start.to_string() + link_end)
        })
        .map(Link)
}
