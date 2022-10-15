use crate::app_struct::app_state::app_command::view_link::payload::ViewLinkStatePayload;
use crate::app_struct::App;
use crate::payloads::ViewLinkPayload;
use anyhow::Result;

use super::actions::choose_name::get_category_and_continue_with_name;
use super::actions::finalize::display_chosen_link;

pub async fn handle_view_link_button_presses(
    app: &mut App,
    view_link_content: ViewLinkPayload,
    view_link_state: ViewLinkStatePayload,
) -> Result<()> {
    match view_link_state {
        ViewLinkStatePayload::WaitingForCategorySelection => {
            get_category_and_continue_with_name(app, view_link_content).await?
        }
        ViewLinkStatePayload::WaitingForNameSelection(category) => {
            display_chosen_link(app, view_link_content, category).await?
        }
    }
    Ok(())
}
