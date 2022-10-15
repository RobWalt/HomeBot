
pub mod payload;

use serde::{Deserialize, Serialize};

use self::payload::MainMenuStatePayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MainMenuState {
    WaitForPayload(MainMenuStatePayload),
}
