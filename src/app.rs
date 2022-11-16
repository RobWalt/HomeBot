use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, VecDeque};
use strum::{Display, EnumString};

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageId};

use crate::links::{Link, LinkCategory, LinkName};
use crate::recipe::Recipe;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct App {
    pub link_storage: BTreeMap<LinkCategory, BTreeMap<LinkName, Link>>,
    pub recipe_storage: Vec<Recipe>,
    pub last_three_recipes: VecDeque<Recipe>,
}

impl App {
    pub fn category_keyboard(&self) -> InlineKeyboardMarkup {
        let buttons = self
            .link_storage
            .keys()
            .map(|LinkCategory(category)| {
                vec![InlineKeyboardButton::callback(
                    category.as_str(),
                    category.as_str(),
                )]
            })
            .collect::<Vec<_>>();
        InlineKeyboardMarkup::new(buttons)
    }

    pub fn name_keyboard_for(&self, category: &LinkCategory) -> InlineKeyboardMarkup {
        let buttons = self
            .link_storage
            .get(category)
            .iter()
            .flat_map(|name_map| {
                name_map
                    .keys()
                    .map(|LinkName(name)| {
                        vec![InlineKeyboardButton::callback(name.as_str(), name.as_str())]
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        InlineKeyboardMarkup::new(buttons)
    }

    pub fn add_link(&mut self, category: LinkCategory, name: LinkName, link: Link) {
        let map = self.link_storage.entry(category).or_default();
        let LinkName(name) = name;
        for i in 1.. {
            let name = LinkName(format!("{name}-{i}"));
            if let Entry::Vacant(entry) = map.entry(name) {
                entry.insert(link);
                break;
            }
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipe_storage.push(recipe);
    }

    pub fn get_random_non_repeating_recipe(&mut self) -> Option<Recipe> {
        self.recipe_storage.shuffle(&mut thread_rng());
        self.recipe_storage
            .iter()
            .find(|recipe| !self.last_three_recipes.contains(recipe))
            .map(|recipe| {
                self.last_three_recipes.push_back(recipe.clone());
                if self.last_three_recipes.len() > 3 {
                    self.last_three_recipes.pop_front();
                }
                recipe
            })
            .cloned()
    }

    pub fn get_one_of_last_three_recipes(&mut self) -> Option<Recipe> {
        self.last_three_recipes.pop_front().map(|recipe| {
            self.last_three_recipes.push_back(recipe.clone());
            recipe
        })
    }
}

#[derive(Debug, Default, Clone)]
pub enum AppState {
    #[default]
    Unloaded,
    Neutral(App),
    WaitingForLinkCommand(App),
    WaitingForRecipeCommand(App),
    LinkStoring(App, Link),
    LinkViewing(App),
    LinkEditWaitingForCommand(App),
    LinkEditing(App),
    RecipeStoring(App),
}

impl AppState {
    pub fn get_app(&self) -> Option<App> {
        match self {
            AppState::Unloaded => None,
            AppState::Neutral(app) => Some(app.clone()),
            AppState::WaitingForLinkCommand(app) => Some(app.clone()),
            AppState::WaitingForRecipeCommand(app) => Some(app.clone()),
            AppState::LinkStoring(app, _) => Some(app.clone()),
            AppState::LinkViewing(app) => Some(app.clone()),
            AppState::LinkEditWaitingForCommand(app) => Some(app.clone()),
            AppState::LinkEditing(app) => Some(app.clone()),
            AppState::RecipeStoring(app) => Some(app.clone()),
        }
    }
}

#[derive(Debug, EnumIter, EnumString, Display)]
pub enum RecipeCommands {
    #[strum(serialize = "Vorschlag")]
    GetRandom,
    #[strum(serialize = "Rezept erstellen")]
    Add,
    #[strum(serialize = "Abbrechen")]
    Dismiss,
}

#[derive(Debug, EnumIter, EnumString, Display)]
pub enum LinkCommands {
    #[strum(serialize = "Suche Link heraus")]
    View,
    #[strum(serialize = "Link bearbeiten")]
    Edit,
    #[strum(serialize = "Abbrechen")]
    Dismiss,
}

#[derive(Debug, EnumIter, EnumString, Display)]
pub enum EditLinkCommads {
    #[strum(serialize = "Link Kategorie bearbeiten")]
    EditCategory,
    #[strum(serialize = "Link Namen bearbeiten")]
    EditName,
    #[strum(serialize = "Abbrechen")]
    Dismiss,
}

pub type MainStateStorage = InMemStorage<AppState>;
pub type MainDialogue = Dialogue<AppState, MainStateStorage>;

#[derive(Default, Debug, Clone)]
pub struct LastMessage(pub Vec<MessageId>);

pub type LastMessageStorage = InMemStorage<LastMessage>;
pub type LastMessageDialogue = Dialogue<LastMessage, LastMessageStorage>;
