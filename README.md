# rime

A TUI app for editing the id3 tags of audio files

------------------------------------------------------------------------------

## What is rime and why does it exist?

It does what it says on the tin; _rime_ is a TUI app that allows the user to
edit the id3 tags of an audio file. This includes, but is not limited to:
- Title
- Album
- Artist
- Track Number
- Release Date
- Etc

## What exactly does it do?

At the moment, it just allows you to browse your local filesystem for MP3 files
and displays the metadata from various id3 tags to the user. In the future, I
intend to implement features such as creating new tags, editing existing tags,
viewing and changing embedded album artwork, bulk metadata edits, and more.

## How can I compile and run it?

First, you need to clone the repo:

```bash
$ git clone https://github.com/Nynergy/rime.git
$ cd rime
```

To build the app and mess around with it, run the following:

```bash
$ cargo build
$ cargo run
```

To install it into your Cargo installation's install root, do:

```bash
$ cargo install --path .
```

Just be sure that your Cargo install root is in your PATH environment variable,
otherwise your shell won't know where to find the installed binary. Once it's
installed, you can run the app by simply running `rime`.

### Keybindings

Key | Action
----|-------
<kbd>q</kbd> / <kbd>Esc</kbd> | quit rime
<kbd>j</kbd> and <kbd>k</kbd> / <kbd>Up</kbd> and <kbd>Down</kbd> | focus list item up/down
<kbd>g</kbd> and <kbd>G</kbd> / <kbd>Home</kbd> and <kbd>End</kbd>| jump to top/bottom of list
<kbd>Space</kbd> / <kbd>Enter</kbd> | select file/directory to view tags

## Now what?

Use it, and properly tag your audio media libraries :)
