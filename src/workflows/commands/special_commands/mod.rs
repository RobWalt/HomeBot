mod start_create_links;
use teloxide::dptree;

use crate::workflows::HandlerType;
pub use start_create_links::CreateLinkStarting;

pub fn create_handler() -> HandlerType {
    dptree::entry().branch(start_create_links::command_handler())
}
