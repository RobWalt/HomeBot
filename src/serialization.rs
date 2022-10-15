use serde::{Deserialize, Serialize};

use telegram_bot::{Supergroup, SupergroupId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSerialize {
    /// Unique identifier for this chat.
    pub id: SupergroupId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for supergroup.
    pub username: Option<String>,
    /// Invite link for this supergroup, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    pub invite_link: Option<String>,
}

impl From<Supergroup> for GroupSerialize {
    fn from(group: Supergroup) -> Self {
        let Supergroup {
            id,
            title,
            username,
            invite_link,
        } = group;
        Self {
            id,
            title,
            username,
            invite_link,
        }
    }
}

impl From<GroupSerialize> for Supergroup {
    fn from(val: GroupSerialize) -> Self {
        let GroupSerialize {
            id,
            title,
            username,
            invite_link,
        } = val;
        Supergroup {
            id,
            title,
            username,
            invite_link,
        }
    }
}
