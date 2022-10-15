pub mod message;
pub mod payload;

use serde::{Deserialize, Serialize};

use self::message::ArchiveLinkStateMessage;
use self::payload::ArchiveLinkStatePayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchiveLinkState {
    WaitForPayload(ArchiveLinkStatePayload),
    WaitForMessage(ArchiveLinkStateMessage),
}
