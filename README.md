[![Build](https://github.com/playonbsd-rs/pobsd-wcip/actions/workflows/rust.yml/badge.svg)](https://github.com/playonbsd-rs/pobsd-wcip/actions/workflows/rust.yml)
[![Clippy](https://github.com/playonbsd-rs/pobsd-wcip/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/playonbsd-rs/pobsd-wcip/actions/workflows/rust-clippy.yml)
[![Crates.io (latest)](https://img.shields.io/crates/v/wcip?style=flat)](https://crates.io/crates/wcip)


## wcip (for What Can I Play)
wcip is a small utility that compares the `steamctl apps list` output to the PlayOnBSD
database, and displays the games you own that can be played on OpenBSD.

### Installation
You need to install `steamctl` (games/steamctl) and openssl (security/openssl/1.1). If you
want to use a different version of openssl, you have to edit the `.cargo/config.toml` accordingly 
until I find a more elegant solution. Otherwise, just use `cargo install wcip` and make sure
you have `$HOME/.cargo/bin` in your `$PATH`

### How to use it
First, you need to configure `steamctl`. Once it is done, it is as simple as:
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
The information provided for each game should be enough
to run it without much trouble.
