# Change Log

Documenting changes between versions beginning from v0.3.0

## v0.10.2

* Reduced binary size - no other changes

## v0.10.1

* Added new command to display the currently loaded environment

## v0.10.0

* SQLite DB support
    * Safir now supports using either a JSON file as storage or an SQLite database
    * Activate this by running `safir mode database` or `safir mode file`

* Environments
    * Safir now supports storing Key-Value pairs in specific environments
    * Calling `safir use [environment-name]` will activate the specific environment
    * New environments are created empty and stored KV pairs will remain in this environment
    * A default environment called `default` is used initially
    * Old stores from previous versions will be ported to this new format automatically

## v0.9.0

**THIS IS A BREAKING CHANGE**

The entire `safir` project has been overhauled and brought back to it's original (and simple) purpose: display Key-value pairs.

When working on the whole project, it became clear to me that it was growing _way_ out of control.
Initially it was to be an in-memory version (similar to Redis) but an on-disk storage solution was developed first.
The in-memory version spawned a whole load of side-projects (`rubin` for one) but in doing so, it became an over-engineered beast.

At it's heart, `safir` was simply meant to be a small program to store key-value pairs and retrieve them later - that's it!
So when I rewrote it in Go for fun, I realised that it was far too complex and could just be a simple command-line tool with simple commands.

So here we are, it's back to it's original "mindset" and is now _far_ simpler, just storing KV pairs on disk.
No additional libraries, no in-memory versions; just a simple KV store.

The other versions are still available but are no longer maintained and this will be the "final" form going forward.

Sorry for any inconvenience and hope you enjoy the more simpler version!

* Overhauled project to be simpler with no crazy, custom-built backend solutions
* No pretty output, just displays the KV in the format `[key]="[value]"`
* No configs, just a JSON file stored at `$HOME/.safirstore/safirstore.json`
* All previous commands still work as they did before (minus the additional ones for in-memory storage)

## v0.8.0

Headless mode!

You can now run safir in "headless" mode which removes all the fancy formatting for output

This will allow you to evaluate safir output directly in the terminal.

This also persist as a config setting saved to `~/.safirstore/safir.cfg`

## v0.7.1

Changes to the internal code structure, no changes to operation

## v0.7.0

Something happened here but not major...

## v0.6.0

Removed the Safir Memcache and moved it to its own project.

Addition of the Memcache functionality was making everything a bit messy to best to separate them both.
They still operate the same, just different projects for each.

Any changes here wil lbe made in the Memcache version also.

* Removal of Memcache
* Technically a reversion but meh

## v0.5.0

Added the ability to operate Safir as a Memcache server (storing contents in-memory instead of on disk).
Requires the [Rubin CLI](https://crates.io/crates/rubin-cli) to be installed.

The Memcache service operates using TCP sockets to communicate with the Rubin server to access and update storage.
It operates on `localhost` or  `127.0.0.1` and on port `9876`.

It works in the same fashion as Safir and does not update the store on disk when activated.

Initialisation and cleanup is handles with the `start` and `stop` commands.

* New memcache option available for using Safir as an in-memory store
* Minor edits and bugfixes

## v0.4.0

**BREAKING CHANGE**: A newer version of `Rubin` is now used which is incompatible with older versions of Safir.
Attempting to use Safir with older stores will now result in a panic.
Save your older store somewhere and manually remove the old `.safirstore` directory.
This should allow continued usage.

* Updated verison of `Rubin` to newer version.

## v0.3.0

No breaking changes, just an update of the backend used for file creation and maintenance.
Due to this allowing more extended work going forward, bumped up to v0.3.0

* Changed backend file handling to use the [Rubin](https://crates.io/crates/rubin) crates instead of a custom solution

