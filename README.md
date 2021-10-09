# Intro
This project is a work-in-progress cross-platform mod manager for the game
[Barony](https://store.steampowered.com/app/371970/Barony/). It aims to provide
a platform-agnostic interface to manage all the barony mods available through its
[Steam Workshop](https://steamcommunity.com/workshop/about/?appid=371970). So
you can easily download Barony mods, whether you are using a [custom Barony build](https://github.com/TurningWheel/Barony)
or the Steam, Epic Games, or GOG version of the game.

# Getting started

## Getting a Steam API key
Currently the app consumes the Steam API directly in order to retrieve data
about the mods. Removing the Steam API key dependency is planned to the next
version, but for now it's required. If you don't already have an API key, you
can head towards https://steamcommunity.com/dev/apikey and get one (if you don't
want to put a domain when creating the key, just use `127.0.0.1` instead).

## Using the mod manager
The interface is meant to be pretty straightforward, and the required steps to
get it up and running are:

- Add Steam API key to the bottom left input.
- Add the Barony folder path to the bottom right input (not the `mods` directory).
- Click refresh and see the mods popping up

After that you can use the search input and the filters to match exactly what
you are looking for and install/uninstall mods using the respective buttons in
their cards. Have a great time modding Barony!

## Security advisory
In order to provide an easier to use interface, the mod manager saves your Barony
path and API key on disk, so you don't need to enter them every time you open
it. So if you are not using your personal computer, you probably want to delete
the folder it saves this data after using it. Its located at:
- `%APPDATA%/barony-mod-manager` on Windows
- `~/.local/share/barony-mod-manager/` on Linux systems

# TODO:
- Remove need for Steam API key
- Handle mods which depends on other mods
- A simple table-like mod view
- Modpack support
- Folder picker widget for choosing the barony folder
- Mod download progress bar
- Wait until the `iced` library improve its `Scrollable` widget, which hopefully
  will make the application use much less resources.
