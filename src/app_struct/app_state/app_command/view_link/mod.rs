pub mod payload;

use serde::{Deserialize, Serialize};

use self::payload::ViewLinkStatePayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewLinkState {
    WaitForPayload(ViewLinkStatePayload),
}
