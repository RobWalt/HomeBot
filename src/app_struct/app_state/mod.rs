pub mod app_command;

use serde::{Deserialize, Serialize};

use self::app_command::AppCommand;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum AppState {
    #[default]
    WaitingForInputs,
    ExecutingCommand(AppCommand),
}
