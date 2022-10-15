pub mod archive_link;
pub mod main_menu;
pub mod view_link;

use serde::{Deserialize, Serialize};

use self::archive_link::ArchiveLinkState;
use self::main_menu::MainMenuState;
use self::view_link::ViewLinkState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppCommand {
    ArchiveLink(ArchiveLinkState),
    MainMenu(MainMenuState),
    ViewLink(ViewLinkState),
}
