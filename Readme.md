SSCCPP, the Simple Switch Cases Configuration PreProcessor
==========================================================

[![Build Status](https://travis-ci.org/krtab/ssccpp.svg?branch=master)](https://travis-ci.org/krtab/ssccpp)
[![Version badge](https://img.shields.io/crates/v/ssccpp.svg)](https://crates.io/crates/ssccpp)

This small utility makes it easy to have a single file that can be adapted to different cases.

Typical use is to **manage dotfiles, by allowing to have only one git branch where each file clearly shows its variation between your different machines.**

It is written in Rust and provides a library as well as two binaries: `ssccpp` and `ssccpp-batch`. `ssccpp`  processes a single file while `ssccpp-batch` process all files in a directory, copying its structure to an other one.

### Table of Content

 - [Concept](#concept)
 - [Examples](#examples)
 - [Install](#install)

Concept
--------

`ssccpp` works on a file where some lines have been turned into switch statements.
These switch statements are recognized by looking for a delimiter (by default `>>>>>>>>` ie. 8 times the '>' character).

A switch block is composed of, in order:

1. One or more "specfic" case(s): `>>>>>>>> ident1, ident2, ...`
2. One "otherwise" case: `>>>>>>>> *`
3. And ending delimiter: `>>>>>>>>`

When reaching such a block, ssccpp will check, for each *specific* case if one of the `ident` matches the searched ident (by default the hostname), and if yes, will emit the text that follows.
When reaching the *otherwise* block, it will emit the text that follows if none of the *specific* statements before were entered. Finally, when reaching the ending delimiter, it will print all following text, until a new block is encountered.

Examples
--------

### First example

```
This line will be displayed everywhere.
>>>>>>>> foo, bar
This line will be displayed on foo and bar only.
>>>>>>>> ga, bu, zo
This line will be displayed on ga, bu and zo, but not foo or bar.
>>>>>>>> foo, zo
This line will be displayed on foo and zo.
>>>>>>>> *
This line will be displayed on anything that is not foo, bar, ga, bu or zo.
>>>>>>>>
This line will be displayed everywhere.
```

### Configuration files example

In this example, the user has a `dotfiles` directory that mimics the structure of their `/home/user` directory:

```
dotfiles/
├── scripts/
│   └── wallpaper.sh
└── .xinitrc
```

The file content is as follow:

**wallpaper.sh**

```
>>>>>>>> laptop
feh --bg-scale 'Images/WallpaperLaptop.png'
>>>>>>>> *
feh --bg-scale 'Images/DefaultWallpaper.png'
```

**.xinitrc**

```
>>>>>>>> desktop
xrandr --dpi 166
>>>>>>>> laptop
xrandr --dpi 96
>>>>>>>>

exec i3
```

Upon executing `ssccpp-batch dotfiles/ /home/user --ident laptop` the files will be placed with the same structure in their home directory and become:

**wallpaper.sh**

```
feh --bg-scale 'Images/WallpaperLaptop.png'
```

**.xinitrc**

```
xrandr --dpi 96

exec i3
```

and upon executing `ssccpp-batch dotfiles/ /home/user --ident desktop` the files will become:

**wallpaper.sh**

```
feh --bg-scale 'Images/DefaultWallpaper.png'
```

**.xinitrc**

```
xrandr --dpi 166

exec i3
```

Install
-------

### Cargo

Build from the sources using cargo

```bash
cargo install ssccpp
```
