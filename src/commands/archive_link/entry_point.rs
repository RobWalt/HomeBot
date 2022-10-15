use crate::app_struct::app_state::app_command::archive_link::message::ArchiveLinkStateMessage;
use crate::app_struct::app_state::app_command::archive_link::payload::ArchiveLinkStatePayload;
use crate::app_struct::App;
use crate::payloads::ArchiveLinkPayload;
use anyhow::Result;

use super::actions::choose_or_create_category::choose_or_create_category;
use super::actions::create_category::start_create_category;
use super::actions::desicison::archive_link_handle_category_descision;
use super::actions::finish::save_link;

pub async fn handle_archive_link_texts(
    app: &mut App,
    archive_link_content: String,
    archive_link_state: ArchiveLinkStateMessage,
) -> Result<()> {
    match archive_link_state {
        ArchiveLinkStateMessage::StartArchiveLink(link) => {
            choose_or_create_category(app, link).await?
        }
        ArchiveLinkStateMessage::CreateCategoryPressed(link, msg_id) => {
            start_create_category(app, archive_link_content, link, msg_id).await?;
        }
        ArchiveLinkStateMessage::AwaitsLinkNameInput(link, category) => {
            save_link(app, archive_link_content, link, category).await?;
        }
    }
    Ok(())
}

pub async fn handle_archive_link_button_presses(
    app: &mut App,
    archive_link_content: ArchiveLinkPayload,
    archive_link_state: ArchiveLinkStatePayload,
) -> Result<()> {
    match archive_link_state {
        ArchiveLinkStatePayload::WaitingForCategoryDesicion(link) => {
            archive_link_handle_category_descision(app, link, archive_link_content).await?;
        }
    }
    Ok(())
}
