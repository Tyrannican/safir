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
  mem     Start or stop the Memcache (in-memory store) service
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Memcache service

Safir offers the ability to run the store as a dedicate memcache service using in-memory storage.

The service runs on `localhost` or `127.0.0.1` on dedicated port `9876`.
Once activated, Safir will continue to act as before expect that all new values added are given to the memcache instead of being saved on disk.

This can be enabled / diabled with the `start` and `stop` commands respectively.
Note that when the memcache service is disabled, ALL data contained within it is lost so use wisely.

In cases where you want to save the contents of the memcache, the `dump` command will allow for the contents to be saved out to disk in JSON format.
This behaves as a snapshot as the contents of the cache persist after usage.

### Usage

Start or stop the Memcache (in-memory store) service

Usage: safir mem <COMMAND>

Commands:
  start  Start the Safir Memcache server
  stop   Stop the Safir Memcache server
  dump   Dump the Safir Memcache server to disk
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

## v0.3.0 -> v0.4.0

v0.4.0 introduces a breaking change which makes it incompatible with older versions of Safir.

To prevent issues, please remove the old `.safirstore/` directory (store the old data somewhere) and re-run Safir.
This should address any issues!
