use std::time::Duration;

use crate::app_struct::App;
use crate::utils::update_with_info_and_delete_after;
use crate::STARTUP_HELP_MESSAGE;
use anyhow::Result;
use telegram_bot::{CanSendMessage, MessageKind, MessageOrChannelPost};

pub async fn send_help(app: &mut App) -> Result<()> {
    let msg = app.api.send(app.group.text(STARTUP_HELP_MESSAGE)).await?;
    if let MessageOrChannelPost::Message(msg) = msg {
        if let MessageKind::Text { data, .. } = msg.kind.clone() {
            update_with_info_and_delete_after(app, msg.chat, msg.id, data, Duration::from_secs(10))
                .await?;
        }
    }
    Ok(())
}
