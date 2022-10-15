use crate::app_struct::app_state::app_command::archive_link::message::ArchiveLinkStateMessage;
use crate::app_struct::app_state::app_command::archive_link::ArchiveLinkState;
use crate::app_struct::app_state::app_command::AppCommand;
use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::payloads::ArchiveLinkPayload;
use crate::types::general::Link;
use anyhow::Result;

pub async fn archive_link_handle_category_descision(
    app: &mut App,
    link: Link,
    archive_link_payload: ArchiveLinkPayload,
) -> Result<()> {
    if let Some(msg_id) = app.active_msg {
        match archive_link_payload {
            ArchiveLinkPayload::LinkCategory(category) => {
                app.set_state(AppState::ExecutingCommand(AppCommand::ArchiveLink(
                    ArchiveLinkState::WaitForMessage(ArchiveLinkStateMessage::AwaitsLinkNameInput(
                        link, category,
                    )),
                )));
                app.update_text_of(msg_id, "Nun gib dem Link bitte einen Namen:")
                    .await?;
            }
            ArchiveLinkPayload::LinkCategoryNew => {
                app.set_state(AppState::ExecutingCommand(AppCommand::ArchiveLink(
                    ArchiveLinkState::WaitForMessage(
                        ArchiveLinkStateMessage::CreateCategoryPressed(link, msg_id),
                    ),
                )));
                app.remove_markup_from(msg_id).await?;
                app.update_text_of(msg_id, "Gib bitte den Namen der neuen Kategorie ein:")
                    .await?;
            }
        }
    }
    Ok(())
}
