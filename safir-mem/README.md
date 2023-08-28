# Safir-mem

Simple in-memory CLI key/value store.

The in-memory version of [Safir](https://crates.io/crates/safir)!

`safir-mem` starts a service which runs on `localhost` or `127.0.0.1` on dedicated port `9876`.

This can be enabled / diabled with the `start` and `stop` commands respectively.
Note that when the memcache service is disabled, ALL data contained within it is lost so use wisely.

In cases where you want to save the contents of the memcache, the `dump` command will allow for the contents to be saved out to disk in JSON format.
This behaves as a snapshot as the contents of the cache persist after usage.

## Install

To install `safir-mem`, run `cargo install safir-mem`.

### Requirements

Using this requires that the [Rubin CLI](https://crates.io/crates/rubin-cli) be installed.

```bash
cargo install rubin-cli
```

## Usage

```bash
In-memory key/value store to share information between shell sessions

Usage: safir-mem <COMMAND>

Commands:
  add     Add a value to the store with the given key
  get     Get a value from the store
  rm      Remove values from the store
  alias   Output the alias command for a key / value pair to be entered into a shell session
  export  Output the export command for a key / value pair to be entered into a shell session
  clear   Clear all keys/values from the store
  start   Start the memcache server
  stop    Stop the memcache server
  dump    Dump contents of memcache to disk
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
