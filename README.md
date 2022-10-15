# HomeBot

A telegram for home management purposes.

The bot works only in groups as it is programmed. Feel free to fork and adjust for your needs.

# Current Features 

### Link Saver 

The detects links in the group chat and archives them for you instantly. The archiving is done in two steps:

1. You choose a category under which the link will be saved 
2. You choose a name for the link 

The links can retrieved by writing the `schmot` command in the chat, which
opens the main menu of the bot. There you can find a button to view the links.
The bot will ask you for the category and name and then writes the link in the
chat.

# Installation 

To install the bot, you just have to build it with cargo and provide your bot's
API token in `$HOME/.local/state/homebot/TOKEN` and information of the group in
`$HOME/.local/state/homebot/GROUP` in the format 

```json
{
  "id": <GROUP_ID>,
  "title": "<GROUP_NAME>",
  "username": null,
  "invite_link": null
}
```
