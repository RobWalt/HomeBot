use serde::{Deserialize, Serialize};

use crate::types::link_store::LinkCategory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewLinkStatePayload {
    WaitingForCategorySelection,
    WaitingForNameSelection(LinkCategory),
}
