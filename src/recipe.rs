use serde::{Deserialize, Serialize};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Recipe(pub String);

#[derive(Debug, Clone, Default)]
pub enum AddRecipeStateWaitingFor {
    #[default]
    RecipeDescription,
}

pub type AddRecipeStateStorage = InMemStorage<AddRecipeStateWaitingFor>;
pub type AddRecipeDialogue = Dialogue<AddRecipeStateWaitingFor, AddRecipeStateStorage>;
