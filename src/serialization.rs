use serde::{Deserialize, Serialize};

use telegram_bot::{Group, GroupId, Supergroup, SupergroupId};

use crate::app_struct::GroupOrSupergroup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupOrSupergroupSerialze {
    Group(GroupSerialize),
    Supergroup(SupergroupSerialize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupergroupSerialize {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSerialize {
    /// Unique identifier for this chat.
    pub id: GroupId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: bool,
    /// Invite link for this group, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    pub invite_link: Option<String>,
}

impl From<GroupOrSupergroup> for GroupOrSupergroupSerialze {
    fn from(group: GroupOrSupergroup) -> Self {
        match group {
            GroupOrSupergroup::Group(g) => Self::Group(g.into()),
            GroupOrSupergroup::Supergroup(s) => Self::Supergroup(s.into()),
        }
    }
}

impl From<GroupOrSupergroupSerialze> for GroupOrSupergroup {
    fn from(group: GroupOrSupergroupSerialze) -> Self {
        match group {
            GroupOrSupergroupSerialze::Group(g) => Self::Group(g.into()),
            GroupOrSupergroupSerialze::Supergroup(s) => Self::Supergroup(s.into()),
        }
    }
}

impl From<Supergroup> for SupergroupSerialize {
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

impl From<SupergroupSerialize> for Supergroup {
    fn from(val: SupergroupSerialize) -> Self {
        let SupergroupSerialize {
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

impl From<GroupSerialize> for Group {
    fn from(group: GroupSerialize) -> Self {
        let GroupSerialize {
            id,
            title,
            all_members_are_administrators,
            invite_link,
        } = group;
        Self {
            id,
            title,
            all_members_are_administrators,
            invite_link,
        }
    }
}

impl From<Group> for GroupSerialize {
    fn from(group: Group) -> Self {
        let Group {
            id,
            title,
            all_members_are_administrators,
            invite_link,
        } = group;
        Self {
            id,
            title,
            all_members_are_administrators,
            invite_link,
        }
    }
}
