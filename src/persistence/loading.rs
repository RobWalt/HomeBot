use std::io::Read;

use anyhow::Result;
use teloxide::types::ChatId;

use crate::app::{App, AppState, MainDialogue};

use super::PATH;

fn read_app(chat_id: ChatId) -> Result<App> {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(PATH);
    path.push(format!("{}.json", chat_id.0));
    let mut f = std::fs::OpenOptions::new().read(true).open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let app = serde_json::from_str(&buf)?;
    Ok(app)
}

pub async fn load_on_first_run(dialogue: MainDialogue) {
    if let Some(AppState::Unloaded) = dialogue.get().await.ok().flatten() {
        log::info!("Loading ...");
        let app = read_app(dialogue.chat_id()).unwrap_or_default();
        _ = dialogue.update(AppState::Neutral(app)).await;
    }
}
