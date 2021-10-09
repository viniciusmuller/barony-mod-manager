<p align="center">
  <img src="barony-mod-manager/resources/img/logo.png" />
  <h1 align="center">Barony Mod Manager</h1>
</p>

# Table of contents

- [Intro](#intro)
- [Getting Started](#getting-started)
  - [Getting a Steam API key](#getting-a-steam-api-key)
  - [Using the Mod Manager](#using-the-mod-manager)
  - [Activating Mods](#activating-mods)
  - [Security Advisory](#security-advisory)
- [TODO:](#todo)

# Intro
This project is a work-in-progress cross-platform mod manager for the game
[Barony](https://store.steampowered.com/app/371970/Barony/). It aims to provide
a platform-agnostic interface to manage all the barony mods available through its
[Steam Workshop](https://steamcommunity.com/workshop/about/?appid=371970). So
you can easily download Barony mods, whether you are using a [custom Barony build](https://github.com/TurningWheel/Barony)
or the Steam, Epic Games, or GOG version of the game.

# Getting Started

## Getting a Steam API key
Currently the app consumes the Steam API directly in order to retrieve data
about the mods. Removing the Steam API key dependency is planned to the next
version, but for now it's required. If you don't already have an API key, you
can head towards https://steamcommunity.com/dev/apikey and get one (if you don't
want to put a domain when creating the key, just use `127.0.0.1` instead).

## Using the Mod Manager
The interface is meant to be pretty straightforward, and the required steps to
get it up and running are:

- Add Steam API key to the bottom left input.
- Add the Barony folder path to the bottom right input (not the `mods` directory).
- Click refresh and see the mods popping up

After that you can use the search input and the filters to match exactly what
you are looking for and install/uninstall mods using the respective buttons in
their cards. Have a great time modding Barony!

## Activating Mods
Since the game does an awesome job at loading/unloading mods at runtime, I don't
think trying to mimic this functionality here is a great deal. So in order to
activate or activate the mods you've downloaded:
- Inside Barony, click `Custom Content`
- Click `local mods` and it will show up all the mods that you've downloaded
    through the mod manager
- `load/unload` the mods that you want to
- Click `start modded game` and be happy!

## Security Advisory
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
