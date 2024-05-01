# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2024-05-01

### Added

- You can now use `SteamClient::new()` to create a client instance without an API key
  - To use an API key, you can take the newly added `SteamClient::from(key)` method
- Dependency `log` to log a warning if you use a `SteamClient` without a valid API key
- New `client.parse_request()` function to reduce code duplication
- `Default` implementations for `SteamClient` and some structs to work with `parse_request`

### Changed

- What were functions previously are now methods of `SteamClient`
  - E.g. `steamr::friends::get_friends()` is now `client.get_friends()`
- Module structure of the crate

### Removed

- `#![deny(rustdoc::missing_doc_code_examples)]` directive (see [issue #101730](https://github.com/rust-lang/rust/issues/101730) for details)
- `ApiClient` trait - it didn't add any use

## [0.3.1] - 2024-03-25

### Changed

- Update reqwest from 0.11.22 to 0.12.1
