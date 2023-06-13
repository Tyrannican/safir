# Change Log

Documenting changes between versions beginning from v0.3.0

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

