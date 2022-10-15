use serde::{Deserialize, Serialize};
use telegram_bot::MessageId;

use crate::types::general::Link;
use crate::types::link_store::LinkCategory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchiveLinkStateMessage {
    StartArchiveLink(Link),
    CreateCategoryPressed(Link, MessageId),
    AwaitsLinkNameInput(Link, LinkCategory),
}
