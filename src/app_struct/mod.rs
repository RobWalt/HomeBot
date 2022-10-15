pub mod app_state;
pub mod serialization;

use telegram_bot::{
    Api, CanSendMessage, DeleteMessage, EditMessageReplyMarkup, EditMessageText, MessageId,
    MessageOrChannelPost, ReplyMarkup, Supergroup, ToMessageId,
};

use crate::types::link_store::LinkStore;

use anyhow::Result;

use self::app_state::AppState;

pub struct App {
    pub api: Api,
    pub group: Supergroup,
    pub active_msg: Option<MessageId>,
    pub link_store: LinkStore,
    pub state: AppState,
}

impl App {
    pub fn new(api: Api, group: Supergroup) -> Self {
        let mut app = Self {
            api,
            group,
            active_msg: Default::default(),
            link_store: Default::default(),
            state: Default::default(),
        };
        // if we fail to load, we just use a blank app
        _ = app.load();
        app
    }

    pub fn set_state(&mut self, new_state: AppState) {
        self.state = new_state;
    }

    fn set_active_msg(&mut self, msg: MessageOrChannelPost) {
        match msg {
            MessageOrChannelPost::Message(msg) => self.active_msg.replace(msg.id),
            MessageOrChannelPost::ChannelPost(post) => self.active_msg.replace(post.id),
        };
    }

    pub async fn remove_markup_from<M: ToMessageId>(&self, msg_id: M) -> Result<()> {
        self.api
            .send(EditMessageReplyMarkup::new(
                self.group.clone(),
                msg_id,
                Option::<ReplyMarkup>::None,
            ))
            .await?;
        Ok(())
    }

    pub async fn remove_active_markup(&self) -> Result<()> {
        if let Some(msg_id) = self.active_msg {
            self.remove_markup_from(msg_id).await?;
        }
        Ok(())
    }

    pub async fn update_markup_of<M: ToMessageId, K: Into<ReplyMarkup>>(
        &self,
        msg_id: M,
        new_keyboard: K,
    ) -> Result<()> {
        self.api
            .send(EditMessageReplyMarkup::new(
                self.group.clone(),
                msg_id,
                Some(new_keyboard),
            ))
            .await?;
        Ok(())
    }

    pub async fn update_active_markup<K: Into<ReplyMarkup>>(&self, new_keyboard: K) -> Result<()> {
        if let Some(msg_id) = self.active_msg {
            self.update_markup_of(msg_id, new_keyboard).await?;
        }
        Ok(())
    }

    pub async fn update_text_of<M: ToMessageId>(&self, msg_id: M, text: &str) -> Result<()> {
        self.api
            .send(EditMessageText::new(self.group.clone(), msg_id, text))
            .await?;
        Ok(())
    }

    pub async fn update_active_text(&mut self, text: &str) -> Result<()> {
        match self.active_msg {
            Some(msg_id) => {
                self.update_text_of(msg_id, text).await?;
            }
            None => {
                self.send_msg(text).await?;
            }
        }
        Ok(())
    }

    pub async fn update_active_text_and_keyboard<K: Into<ReplyMarkup>>(
        &mut self,
        text: &str,
        new_keyboard: K,
    ) -> Result<()> {
        self.update_active_text(text).await?;
        self.update_active_markup(new_keyboard).await?;
        Ok(())
    }

    pub async fn send_msg_with_keyboard<M: Into<ReplyMarkup>>(
        &mut self,
        msg_text: &str,
        keyboard: M,
    ) -> Result<()> {
        let msg = self
            .api
            .send(self.group.clone().text(msg_text).reply_markup(keyboard))
            .await?;
        self.set_active_msg(msg);
        Ok(())
    }

    pub async fn delete_msg<M: ToMessageId>(&self, msg_id: M) -> Result<()> {
        self.api
            .send(DeleteMessage::new(self.group.clone(), msg_id))
            .await?;
        Ok(())
    }

    pub async fn delete_active_msg(&mut self) -> Result<()> {
        if let Some(msg_id) = self.active_msg.take() {
            self.delete_msg(msg_id).await?;
        }
        Ok(())
    }

    pub async fn send_msg(&mut self, msg_text: &str) -> Result<()> {
        self.delete_active_msg().await?;
        let msg = self.api.send(self.group.clone().text(msg_text)).await?;
        self.set_active_msg(msg);
        Ok(())
    }

    pub async fn send_no_reply_msg(&self, msg_text: &str) -> Result<()> {
        self.api.send(self.group.clone().text(msg_text)).await?;
        Ok(())
    }
}
