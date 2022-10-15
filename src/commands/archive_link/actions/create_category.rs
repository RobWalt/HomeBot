use telegram_bot::MessageId;

use crate::app_struct::app_state::app_command::archive_link::message::ArchiveLinkStateMessage;
use crate::app_struct::app_state::app_command::archive_link::ArchiveLinkState;
use crate::app_struct::app_state::app_command::AppCommand;
use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::types::general::Link;
use crate::types::link_store::LinkCategory;

use anyhow::Result;

pub async fn start_create_category(
    app: &mut App,
    category_name: String,
    link: Link,
    msg_id: MessageId,
) -> Result<()> {
    app.set_state(AppState::ExecutingCommand(AppCommand::ArchiveLink(
        ArchiveLinkState::WaitForMessage(ArchiveLinkStateMessage::AwaitsLinkNameInput(
            link,
            LinkCategory::new(category_name),
        )),
    )));
    app.update_text_of(msg_id, "Nun gib dem Link bitte einen Namen:")
        .await?;
    Ok(())
}
