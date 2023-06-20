# NVAPI

[![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

`nvapi` provides access to NVIDIA driver functionality on Windows.

## [Documentation][docs]

See the [documentation][docs] for up to date information.

This library is provided as 3 separate crates:
- [`nvapi-hi`](https://docs.rs/nvapi-hi/) is usually what you'd want to use as it takes care of most of the internals for you, and things make more sense
- [`nvapi`](https://docs.rs/nvapi/) is the middle ground, allows you to get a bit more dirty with the NVAPI and there are invariants that you will need to uphold otherwise crashing and/or unexpected behavior is expected
- [`nvapi-sys`](https://docs.rs/nvapi-sys/) expose unsafe bindings to the C NVAPI, you can do pretty much anything but you absolutely need to know what you're doing

[release-badge]: https://img.shields.io/crates/v/nvapi.svg?style=flat-square
[cargo]: https://crates.io/crates/nvapi
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: https://docs.rs/nvapi-hi/
[license-badge]: https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square
[license]: https://github.com/arcnmx/nvapi-rs/blob/master/COPYING
