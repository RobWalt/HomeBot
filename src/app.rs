use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;
use teloxide::types::MessageId;

use crate::links::{Link, LinkCategory, LinkName};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct App {
    pub link_storage: BTreeMap<LinkCategory, BTreeMap<LinkName, Link>>,
}

#[derive(Debug, Default, Clone)]
pub enum AppState {
    #[default]
    Unloaded,
    Neutral(App),
    LinkStoring(App, Link),
    LinkViewing(App),
}

impl AppState {
    pub fn get_app(&self) -> Option<App> {
        match self {
            AppState::Unloaded => None,
            AppState::Neutral(app) => Some(app.clone()),
            AppState::LinkStoring(app, _) => Some(app.clone()),
            AppState::LinkViewing(app) => Some(app.clone()),
        }
    }
}

pub type MainStateStorage = InMemStorage<AppState>;
pub type MainDialogue = Dialogue<AppState, MainStateStorage>;

#[derive(Default, Debug, Clone)]
pub struct LastMessage(pub Vec<MessageId>);

pub type LastMessageStorage = InMemStorage<LastMessage>;
pub type LastMessageDialogue = Dialogue<LastMessage, LastMessageStorage>;
