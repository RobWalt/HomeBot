use std::io::Write;

use anyhow::Result;
use teloxide::types::ChatId;

use crate::app::{App, MainDialogue};

use super::PATH;

fn write_app(app: &App, chat_id: ChatId) -> Result<()> {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(PATH);
    path.push(format!("{}.json", chat_id.0));
    match path.parent() {
        Some(parent) if !parent.exists() => {
            std::fs::create_dir_all(parent)?;
        }
        _ => {}
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    let app_data = serde_json::to_string(&app)?;
    f.write_all(app_data.as_bytes())?;
    Ok(())
}

pub async fn save_every_run(dialogue: MainDialogue) {
    log::info!("Saving ...");
    if let Some(app) = dialogue
        .get()
        .await
        .ok()
        .flatten()
        .and_then(|app_state| app_state.get_app())
    {
        if let Err(e) = write_app(&app, dialogue.chat_id()) {
            log::error!("Error while saving: {e}");
        }
    }
}
