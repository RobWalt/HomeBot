use std::collections::HashMap;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::general::Link;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkCategory(String);

impl LinkCategory {
    pub fn new(category: String) -> Self {
        Self(category.chars().take(30).collect())
    }
}

impl Display for LinkCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkName(String);

impl LinkName {
    pub fn new(name: String) -> Self {
        Self(name.chars().take(30).collect())
    }
}

impl Display for LinkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type LinkStore = HashMap<LinkCategory, HashMap<LinkName, Link>>;
