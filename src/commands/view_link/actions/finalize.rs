use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::payloads::ViewLinkPayload;
use crate::types::general::Link;
use crate::types::link_store::LinkCategory;
use anyhow::Result;

pub async fn display_chosen_link(
    app: &mut App,
    view_link_content: ViewLinkPayload,
    category: LinkCategory,
) -> Result<()> {
    if let ViewLinkPayload::NameChosen(name) = view_link_content {
        if let Some(Link(url)) = app
            .link_store
            .get(&category)
            .and_then(|link_list| link_list.get(&name))
            .cloned()
        {
            app.delete_active_msg().await?;
            app.send_no_reply_msg(&url).await?;
            app.set_state(AppState::WaitingForInputs);
        }
    }
    Ok(())
}
