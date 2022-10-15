use crate::app_struct::app_state::app_command::archive_link::payload::ArchiveLinkStatePayload;
use crate::app_struct::app_state::app_command::archive_link::ArchiveLinkState;
use crate::app_struct::app_state::app_command::AppCommand;
use crate::app_struct::app_state::AppState;
use crate::app_struct::App;
use crate::payloads::{ArchiveLinkPayload, Payload};
use crate::types::callback::MyCallbackFormat;
use crate::types::general::Link;
use crate::utils::get_chunk_size;
use anyhow::Result;
use telegram_bot::{
    CanSendMessage, InlineKeyboardButton, InlineKeyboardMarkup, MessageOrChannelPost,
};

pub async fn choose_or_create_category(app: &mut App, link: Link) -> Result<()> {
    let categories = app.link_store.keys().collect::<Vec<_>>();
    let chunk_size = get_chunk_size(&categories);
    let buttons = categories
        .chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .map(|category| (**category).clone())
                .map(|category| {
                    let button_name = category.to_string();
                    let cb = MyCallbackFormat::new(Payload::ArchiveLink(
                        ArchiveLinkPayload::LinkCategory(category),
                    ));
                    InlineKeyboardButton::callback(button_name, cb.callback_text())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let create_button_cb =
        MyCallbackFormat::new(Payload::ArchiveLink(ArchiveLinkPayload::LinkCategoryNew));
    let create_button = InlineKeyboardButton::callback("Neu", create_button_cb.callback_text());

    let mut keyboard = InlineKeyboardMarkup::new();

    for button_row in buttons.iter() {
        keyboard.add_row(button_row.to_vec());
    }
    keyboard.add_row(vec![create_button]);

    let msg = app
        .api
        .send(
            app.group
                .text("Waehle die Kategorie unter welcher der Link gespeichert werden soll:")
                .reply_markup(keyboard),
        )
        .await?;

    if let MessageOrChannelPost::Message(msg) = msg {
        let msg_id = msg.id;
        app.active_msg.replace(msg_id);
    }

    app.set_state(AppState::ExecutingCommand(AppCommand::ArchiveLink(
        ArchiveLinkState::WaitForPayload(ArchiveLinkStatePayload::WaitingForCategoryDesicion(link)),
    )));

    Ok(())
}
