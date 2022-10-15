use serde::{Deserialize, Serialize};

use crate::types::link_store::{LinkCategory, LinkName};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Payload {
    MainMenu(MainMenuPayload),
    ArchiveLink(ArchiveLinkPayload),
    ViewLink(ViewLinkPayload),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MainMenuPayload {
    Links,
    Dismiss,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchiveLinkPayload {
    LinkCategory(LinkCategory),
    LinkCategoryNew,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewLinkPayload {
    CategoryChosen(LinkCategory),
    NameChosen(LinkName),
}
