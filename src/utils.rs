use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use telegram_bot::{DeleteMessage, EditMessageText, Supergroup, ToChatRef, ToMessageId};

use tokio::time::delay_for;

use crate::app_struct::App;
use crate::serialization::GroupSerialize;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

fn app_data_dir_path() -> Result<PathBuf> {
    let local_app_data_dir_path =
        Path::new(".local").join(Path::new("state").join(Path::new("homebot")));
    let home_path = dirs::home_dir().ok_or_else(|| anyhow!("Home dir not found, can't save!"))?;
    Ok(home_path.join(local_app_data_dir_path))
}

fn join_with_app_data_dir(file: &str) -> Result<PathBuf> {
    app_data_dir_path().map(|p| p.join(file))
}

pub fn token_path() -> Result<PathBuf> {
    join_with_app_data_dir("TOKEN")
}

pub fn group_path() -> Result<PathBuf> {
    join_with_app_data_dir("GROUP")
}

pub fn app_state_path() -> Result<PathBuf> {
    join_with_app_data_dir("APPSTATE")
}

pub fn read_data(path: PathBuf) -> Result<String> {
    let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

pub fn parse_group(string_data: String) -> Result<Supergroup> {
    serde_json::from_str::<GroupSerialize>(&string_data)
        .map(Into::<Supergroup>::into)
        .map_err(Error::from)
}

pub fn save_data(string_data: String, path: PathBuf) -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all(string_data.as_bytes()).map_err(Error::from)
}

pub async fn update_with_info_and_delete_after<C: ToChatRef + Clone, M: ToMessageId + Clone>(
    app: &mut App,
    chat: C,
    msg_id: M,
    text: String,
    duration: Duration,
) -> Result<()> {
    let seconds = duration.as_secs();
    let text = format!("{text}\n\n(Wird in {seconds}s geloescht)");
    app.api
        .send(EditMessageText::new(chat.clone(), msg_id.clone(), text))
        .await?;
    delay_for(Duration::from_secs(seconds)).await;
    app.api.send(DeleteMessage::new(chat, msg_id)).await?;
    Ok(())
}

pub fn get_chunk_size<T>(names: &[T]) -> usize {
    let amount = names.len();
    match () {
        _ if (0..10).contains(&amount) => 1,
        _ if (10..20).contains(&amount) => 2,
        _ => 3,
    }
}
