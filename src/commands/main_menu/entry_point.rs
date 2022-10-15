use crate::app_struct::app_state::app_command::main_menu::payload::MainMenuStatePayload;
use crate::app_struct::App;
use crate::payloads::MainMenuPayload;
use anyhow::Result;

use super::actions::selection::main_menu_handler;

pub async fn handle_main_menu_button_presses(
    app: &mut App,
    main_menu_content: MainMenuPayload,
    main_menu_state: MainMenuStatePayload,
) -> Result<()> {
    match main_menu_state {
        MainMenuStatePayload::WaitingForButtonPress => {
            main_menu_handler(app, main_menu_content).await?;
        }
    }
    Ok(())
}
