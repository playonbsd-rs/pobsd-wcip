[![check](https://github.com/playonbsd-rs/pobsd-wcip/actions/workflows/check.yml/badge.svg)](https://github.com/playonbsd-rs/pobsd-wcip/actions/workflows/check.yml)
[![test](https://github.com/playonbsd-rs/pobsd-wcip/actions/workflows/test.yml/badge.svg)](https://github.com/playonbsd-rs/pobsd-wcip/actions/workflows/test.yml)
[![Crates.io (latest)](https://img.shields.io/crates/v/wcip?style=flat)](https://crates.io/crates/wcip)
[![codecov](https://codecov.io/gh/playonbsd-rs/pobsd-wcip/branch/main/graph/badge.svg?token=1QNYI0Q32N)](https://codecov.io/gh/playonbsd-rs/pobsd-wcip)


## wcip (for What Can I Play)
wcip is a small utility that compares the `steamctl apps list` output to the PlayOnBSD
database, and displays the games you own that can be played on OpenBSD.

### Installation
You need to install `steamctl` (games/steamctl) and openssl (security/openssl/1.1).

You can install `wcip` using `cargo install wcip` with following env variables (to adapt
according to your openssl version if different from the one indicated above):
```
OPENSSL_LIB_DIR=/usr/local/lib/eopenssl11/
OPENSSL_INCLUDE_DIR=/usr/local/include/eopenssl11/ 
```

Then, make sure you have `$HOME/.cargo/bin` in your `$PATH`

### Configuration
The configuraiton file for `wcip` is located at `$HOME/.config/wcip/config.toml`.

```toml
download_path = "/some/absolute_path/"
steam_id = "not_secre"
steam_key = "super_secret"
```

The three parameters are all optionals.

If `download_path` is set, it will be used instead of the `<PATH>` 
placeholder in the install command.

If `steam_id` and `steam_key` are both set, `wcip` will use the Steam
Web API instead of steamctl to get the list of owned games. Otherwise,
you need to configure `steamctl`.

### How to use it
`wcip` provides two subcommands: `ls` and `tui`.

The `ls` subcommand lists of owned games running on OpenBSD in the terminal:
```
$ wcip  
Loading steam data. Please wait.
------------------------------------
 Gemini Rue
 Hints: None
 Install: steamctl depot download -a 80310 -o <PATH>
 engine: AGS
 runtime: scummvm
 url: https://store.steampowered.com/app/80310
------------------------------------
 Primordia
 Hints: None
 Install: steamctl depot download -a 227000 -o <PATH>
 engine: AGS
 runtime: AGS
 url: https://store.steampowered.com/app/227000/Primordia/
------------------------------------
 The Blackwell Epiphany
 Hints: None
 Install: steamctl depot download -a 236930 -o <PATH>
 engine: AGS
 runtime: AGS
 url: https://store.steampowered.com/app/236930
------------------------------------
 The Shivah
 Hints: None
 Install: steamctl depot download -a 252370 -o <PATH>
 engine: AGS
 runtime: scummvm
 url: https://store.steampowered.com/app/252370
------------------------------------
```

The `tui` subcommand give a TUI to browse owned games running on OpenBSD in the terminal:
```
╭Games────────────────────────────────────────────────────────────────╮╭Details──────────────────────────────────────────────────────────────╮
│ ╭ Search ─────────────────────────────────────────────────────────╮ ││ DREAMS IN THE WITCH HOUSE                                           │
│ │                                                                 │ ││                                                                     │
│ ╰─────────────────────────────────────────────────────────────────╯ ││ Engine: AGS                                                         │
│ The Blackwell Epiphany                                              ││ Runtime: scummvm                                                    │
│ Dreams in the Witch House                                           ││ Stores:                                                             │
│ The Excavation of Hob's Barrow                                      ││ - https://www.gog.com/en/game/dreams_in_the_witch_house             │
│ Gemini Rue                                                          ││ - https://store.steampowered.com/app/1902850/Dreams_in_the_Witch_Ho │
│ Northgard                                                           ││ Hints: with scummvm 2.7.0 has segfault when looking down platform i │
│ Primordia                                                           ││ Year: 2023                                                          │
│ The Shivah                                                          ││ Developer: Atom Brain Games                                         │
│ Strangeland                                                         ││ Publisher: Bonus Stage Publishing                                   │
│ Technobabylon                                                       ││ Version: 1.04                                                       │
│ Unavowed                                                            ││ Status: runs with rare bugs, unclear if game breaking (2023-03-05)  │
│                                                                     ││ Added: 2023-03-05                                                   │
│                                                                     ││ Last updated: 2023-03-05                                            │
│                                                                     ││                                                                     │
│                                                                     ││                                                                     │
│                                                                     ││ ┏GENRES━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ │
│                                                                     ││ ┃point and click adventure                                        ┃ │
│                                                                     ││ ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ │
│                                                                     ││ ┏TAGS━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ │
│                                                                     ││ ┃horror Lovecraftian                                              ┃ │
│                                                                     ││ ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ │
│                                                                     ││                                                                     │
│                                                                     ││ Key bindings                                                        │
│                                                                     ││ s:    Search mode                                                   │
│                                                                     ││ TAB:  On search mode, change search mode (name/tag/genre)           │
│                                                                     ││ ESC:  On search mode, back to list mode                             │
│                                                                     ││ UP:   Previous on the list                                          │
│                                                                     ││ DOWN: Next on the list                                              │
│                                                                     ││ k:    On list mode, previous on the list                            │
│                                                                     ││ j:    On list mode, next on the list                                │
│                                                                     ││ q:    On list mode, exit                                            │
╰─────────────────────────────────────────────────────────────────────╯╰─────────────────────────────────────────────────────────────────────╯
```
