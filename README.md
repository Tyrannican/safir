# Safir

Simple CLI key/value store.

Store key/value pairs in the terminal and retrieve them later for use like in different shell sessions.

## Install

To install `safir`, run `cargo install safir`.

To build from source, clone the repository and run:

```bash
cargo build --release
```

Then move the binary to somewhere in your `$PATH`

## Usage

When `safir` is run, it creates a store file in your `$HOME` directory (`$HOME/.safirstore/safirstore.json`).

Run `safir --help` for usage:

```bash
Key/Value store to share information between shell sessions

Usage: safir <COMMAND>

Commands:
  add     Add a value to the store with the given key
  get     Get a value from the store
  rm      Remove values from the store
  alias   Output the alias command for a key / value pair to be entered into a shell session
  export  Output the export command for a key / value pair to be entered into a shell session
  clear   Clear all keys/values from the store
  purge   Purges the .safirstore directory, removing it and its contents
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## v0.3.0 -> v0.4.0

v0.4.0 introduces a breaking change which makes it incompatible with older versions of Safir.

To prevent issues, please remove the old `.safirstore/` directory (store the old data somewhere) and re-run Safir.
This should address any issues!
