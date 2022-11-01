mod create_links;
use teloxide::dptree;

use crate::workflows::HandlerType;

pub fn create_handler() -> HandlerType {
    dptree::entry().branch(create_links::command_handler())
}
