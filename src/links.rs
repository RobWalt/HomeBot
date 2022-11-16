use serde::{Deserialize, Serialize};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawLinkContainingMessage(pub String);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link(pub String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LinkName(pub String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LinkCategory(pub String);

#[derive(Debug, Clone, Default)]
pub enum NewLinkStateWaitingFor {
    #[default]
    Entry,
    Category,
    NewCategory,
    Name(LinkCategory),
}

pub type NewLinkStateStorage = InMemStorage<NewLinkStateWaitingFor>;
pub type NewLinkDialogue = Dialogue<NewLinkStateWaitingFor, NewLinkStateStorage>;

#[derive(Debug, Clone, Default)]
pub enum ViewLinkStateWaitingFor {
    #[default]
    Category,
    Name(LinkCategory),
}

pub type ViewLinkStateStorage = InMemStorage<ViewLinkStateWaitingFor>;
pub type ViewLinkDialogue = Dialogue<ViewLinkStateWaitingFor, ViewLinkStateStorage>;

#[derive(Debug, Clone, Default)]
pub enum EditLinkStateWaitingFor {
    #[default]
    ChoosePath,
    Category,
    CategoryNewName(LinkCategory),
    NameCategory,
    Name(LinkCategory),
    NameNewName(LinkCategory, LinkName),
}

pub type EditLinkStateStorage = InMemStorage<EditLinkStateWaitingFor>;
pub type EditLinkDialogue = Dialogue<EditLinkStateWaitingFor, EditLinkStateStorage>;
