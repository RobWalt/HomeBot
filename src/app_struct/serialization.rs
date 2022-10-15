use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use telegram_bot::Supergroup;

use crate::serialization::GroupSerialize;
use crate::types::link_store::LinkStore;
use crate::utils::app_state_path;
use crate::utils::read_data;
use crate::utils::save_data;

use super::App;

#[derive(Serialize, Deserialize)]
pub struct AppSerialize {
    pub group: GroupSerialize,
    pub link_store: LinkStore,
}

impl App {
    pub fn save(&self) -> Result<()> {
        let Self {
            group, link_store, ..
        } = self;

        let group = GroupSerialize::from(group.clone());
        let data = AppSerialize {
            group,
            link_store: link_store.clone(),
        };

        let save_path = app_state_path()?;
        let dir_path = save_path
            .parent()
            .ok_or_else(|| anyhow!("APPSTATE parent dir not found"))?;
        if !dir_path.exists() {
            std::fs::create_dir_all(dir_path)?;
        }
        let data = serde_json::to_string(&data)?;
        save_data(data, save_path)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let data = read_data(app_state_path()?)?;
        let AppSerialize { group, link_store } = serde_json::from_str(&data)?;
        let group = Supergroup::from(group);
        self.group = group;
        self.link_store = link_store;
        Ok(())
    }
}
