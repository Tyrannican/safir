# Change Log

Documenting changes between versions beginning from v0.3.0

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

