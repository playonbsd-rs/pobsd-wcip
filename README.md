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
