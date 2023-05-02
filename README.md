# Safir

Simple CLI key/value store.

Store key/value pairs in the terminal and retrieve them later for use like in different shell sessions.

## Install

To install `safir`, run `cargo install safir`.

To build from source, clone the repository and run:

```bash
cargo build --release
```

Then move the binary in `<repo>/target/release/safir` to somewhere in your `$PATH`

## Usage

When `safir` is run, it creates a store file in your `$HOME` directory (`$HOME/.safirstore/safirstore.json`).
`safir` comes with several options:

* `add`: This adds a Key / Value pair to the store
* `get`: Retreives a value from the store for a given key
* `rm`: Removes a value from the store with a given key
* `alias`: Outputs a list of keys in the `alias` command format so the user can copy/paste them into a session to set them
    * e.g. `safir alias build`
    * Will output: `alias build="whatever you set as the value for build"`
* `export`: Same as `alias` but uses export syntax
* `clear`: Clears the contents of the store
* `purge`: Clears and delete the `.safirstore` directory

Run `safir --help` for more information
