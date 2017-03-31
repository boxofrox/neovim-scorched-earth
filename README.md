# neovim-scorched-earth

A stupid proof-of-concept for a Neovim plugin written in Rust using
[daa84/neovim-lib](https://github.com/daa84/neovim-lib).

## What does it do?

Scorched Earth highlights the region of text that was touched by the cursor
while in Insert or Replace modes.

![demo](https://boxofrox.github.com/neovim-scorched-earth/assets/images/demo.gif)

## Why... this?

1.  This is my first Neovim plugin in Rust.

2.  A highlight plugin is new territory and interests me, so I'm less likely to
    give up on it.

3.  I'm unfamiliar with the nuances of Msgpack RPC communications with Neovim
    using neovim-lib.  The way was fraught with danger.  This project was just
    deep enough to burn myself a few times tinkering with different designs,
    and shallow enough that I didn't invest a depressing amount of work that
    needed refactoring.

4.  Experiment with a directory structure to combine the rust source and
    vimscript plugin.

## Try it out

1.  It's dangerous to go alone.  Take this!  https://www.rustup.rs/

2.  Use the stable rust compiler.

```sh
rustup install stable
rustup default stable

```

2.  Fetch the plugin.

```sh
$ git clone https://github.com/boxofrox/neovim-scorched-earth.git
```

3.  Build the binary portion of the plugin.

```sh
$ cd neovim-scorched-earth
$ cargo build --release
```

4.  Test it out in a fresh instance of Neovim. *(If Windows requires any
    changes, open an issue!)*

```sh
nvim -u ./init.vim --noplugin -c ":ScorchedEarthConnect"
```

The `ScorchedEarthConnect` command spawns the Rust plugin in a separate process and
establishes a channel.

## Todo

- [ ] Add vimscript variable `g:scorched_earth_syntax_group` to select
      highlight color using existing syntax groups.

- [x] Add vimscript variable `g:scorched_earth_program` to specify location of Rust
      plugin binary.  Will facilitate testing development binaries.
