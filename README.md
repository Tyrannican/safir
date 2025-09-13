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
  get     Get values from the store
  rm      Remove values from the store
  alias   Output the alias command for  key / value pairs
  export  Output the export command for a key / value pairs
  mode    Sets the mode for Safir (active on the next run of Safir)
  list    List all values in the store
  clear   Clear all keys/values from the store
  purge   Purges the .safirstore directory, removing it and its contents
  use     Use / create an environment to store key / value pairs
  env     Display the current loaded environment
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Environments

Safir now supports storing KV pairs in specific environments which will declutter the `list` command and also keep all related KV pairs in one single place.

When Safir is loaded initially, it creates a new `default` environment (or loads an existing one) and uses that unlesss you switch environment.

Stores from older versions of Safir will automatically be ported over to this new format so there is no worry of lost data!

## Examples

Adding a key and value to the store:

```bash
safir add api_key "api_key_value"
```

Retrieving a value from the store:

```bash
safir get api_key
# api_key="api_key_value"
```

Removing a value from the store:

```bash
safir rm api_key
```

List all values in the store:

```bash
safir list

# api_key="api_key_value"
# another_api_key="another_value"
```

Exporting a value:

```bash
safir export api_key
# export api_key="api_key_value"

eval $(safir export api_key) # <-- Will export the value to the current shell
```

Aliasing a value:

```bash
safir alias long_command
# alias long_command="cd build/ && make && sudo make install"

eval $(safir alias long_command) # <-- Will alias the command in the current shell
```

Clear the store:

```bash
safir clear
# Will remove all contents in the store
```

Purge the store (remove EVERYTHING `safir` related)

```bash
safir purge # Will remove the .safirstore directory
```

Setting an environment:

```bash
safir use [environment-name] # This will create an empty environment or load an existing one
```

Switching between `file` and `database` mode:

```bash
safir mode file # Switch to using a JSON file for storage

safir mode database # Switch to using an SQLite database for storage
```

Displaying the currently loaded environment:

```bash
safir env # Will display the currently loaded environment
```
