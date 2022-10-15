use serde::{Deserialize, Serialize};

use crate::types::general::Link;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchiveLinkStatePayload {
    WaitingForCategoryDesicion(Link),
}
