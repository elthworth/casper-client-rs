# Changelog

All notable changes to this project will be documented in this file.  The format is based on [Keep a Changelog].

[comment]: <> (Added:      new features)
[comment]: <> (Changed:    changes in existing functionality)
[comment]: <> (Deprecated: soon-to-be removed features)
[comment]: <> (Removed:    now removed features)
[comment]: <> (Fixed:      any bug fixes)
[comment]: <> (Security:   in case of vulnerabilities)



## [Unreleased]

### Fixed
* Restore the `-p` short form of `--public-key` arg for `get-account-info` and `account-address` subcommands.



## [1.4.3] - 2021-12-06

### Changed
* Update dependencies, in particular `casper-types` to use fixed checksummed-hex format.



## [1.4.2] - 2021-11-13

### Changed
* Support checksummed-hex encoding of various types like `PublicKey` and `AccountHash`.



## [1.4.1] - 2021-10-23

No changes.



## [1.4.0] - 2021-10-21 [YANKED]

### Added
* RPM package build and publish.
* New client binary command `get-validator-changes` that returns status changes of active validators.
* Add `keygen::generate_files` to FFI.

### Changed
* Support building and testing using stable Rust.
* Support `URef`, `PublicKey` and `AccountHash` as transfer targets for `transfer` and `make-transfer`.

### Fixed
* Stop silently ignoring parse errors for `--session-args-complex` or `--payment-args-complex`.



## [1.3.4] - 2021-10-14

No changes.



## [1.3.3] - 2021-10-14

No changes.



## [1.3.2] - 2021-08-02

No changes.



## [1.3.1] - 2021-07-25

No changes.



## [1.3.0] - 2021-07-20

### Added
* Add support for retrieving historical auction information via the addition of an optional `--block-identifier` arg in the `get-auction-info` subcommand.

### Changed
* Change `account-address` subcommand to output properly formatted string.
* Change `put-deploy` and `make-deploy` subcommands to support transfers.
* Change `make-deploy`, `make-transfer` and `sign-deploy` to not overwrite files unless `--force` is passed.
* Change `make-deploy`, `make-transfer` and `sign-deploy` to use transactional file writing for enhanced safety and reliability.
* Update pinned version of Rust to `nightly-2021-06-17`
* Change the Rust interface of the client library to expose `async` functions, instead of running an executor internally.



## [1.2.1] - 2021-07-17

### Changed
* Minor cleanup.



## [1.2.0] - 2021-05-27

### Added
* Support multisig transfers via new `make-transfer` subcommand.

### Changed
* Change to Apache 2.0 license.
* Make `--transfer-id` a required argument of the relevant subcommands.
* Reduce deploy default time-to-live to 30 minutes.



## [1.1.1] - 2021-04-19

No changes.



## [1.1.0] - 2021-04-13 [YANKED]

No changes.



## [1.0.1] - 2021-04-08

### Changed
* Fail if creating a deploy greater than 1 MiB.



## [1.0.0] - 2021-03-30

### Added
* Initial release of client compatible with Casper mainnet.



[Keep a Changelog]: https://keepachangelog.com/en/1.0.0
[unreleased]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.3...main
[1.4.3]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.2...v1.4.3
[1.4.2]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.1...v1.4.2
[1.4.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.0...v1.4.1
[1.4.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.4...v1.4.0
[1.3.4]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.3...v1.3.4
[1.3.3]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.2...v1.3.3
[1.3.2]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.2.1...v1.3.0
[1.2.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.1.1...v1.2.0
[1.1.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/casper-ecosystem/casper-client-rs/tree/v1.0.0
