# HomeBot

A telegram for home management purposes.

The bot works only in groups as it is programmed. Feel free to fork and adjust for your needs.

# Current Features 

### Help

The command `/help` displays a help menu.

### Link Saver 

The bot detects links in any chat and archives them for you instantly. The archiving is done in two steps:

1. You choose a category under which the link will be saved 
2. You choose a name for the link 

The links can retrieved by the `/links` command, which guides you through a dialogue to choose the desired link.

# Installation 

To install the bot, you just have to build it with cargo and provide your bot's
API token in your environment under the variable `TELOXIDE_TOKEN`
