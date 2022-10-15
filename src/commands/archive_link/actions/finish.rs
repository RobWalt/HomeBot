use std::collections::HashMap;
use std::time::Duration;

use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::types::general::Link;
use crate::types::link_store::{LinkCategory, LinkName};
use anyhow::Result;
use tokio::time::delay_for;

pub async fn save_link(
    app: &mut App,
    link_name: String,
    link: Link,
    link_category: LinkCategory,
) -> Result<()> {
    app.set_state(AppState::WaitingForInputs);

    let name_link_map = app
        .link_store
        .entry(link_category)
        .or_insert_with(HashMap::default);

    let link_name = LinkName::new(link_name);
    let unique_link_name = (!name_link_map.contains_key(&link_name))
        .then(|| link_name.clone())
        .unwrap_or_else(|| {
            let mut i = 0;
            let link_name = link_name.to_string();
            while name_link_map.contains_key(&LinkName::new(format!("{i}-{link_name}"))) {
                i += 1;
            }
            LinkName::new(format!("{i}-{link_name}"))
        });
    name_link_map.insert(unique_link_name, link);

    app.send_msg("Link erfolgreich hinzugefuegt!").await?;
    delay_for(Duration::from_secs(5)).await;
    app.delete_active_msg().await?;

    Ok(())
}
