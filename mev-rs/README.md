<img align="right" width="150" height="150" top="100" src="./assets/mev-rs.png">

# mev-rs • [![ci](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml) ![license](https://img.shields.io/badge/License-MIT-green.svg?label=license) ![crates.io](https://img.shields.io/crates/v/mev-rs)


Modern and maximally-minimal rust tooling for MEV.


### Usage

Install [rust](https://www.rust-lang.org/tools/install) if it's not already installed.

Then you can add `mev_rs` to your `Cargo.toml`:

```toml
[dependencies]
mev_rs = "0.1.0"
```


### Blueprint

```ml
.
├─ src
│  ├─ lib.rs — Exported modules with a re-exported prelude.
│  ├─ numeric.rs — Refactored functions for numeric operations.
│  ├─ relayer.rs — Wrappers for network requests.
│  ├─ telemetry.rs — Telemetry for verbose logging.
│  ├─ uniswap.rs — Uniswap library.
│  └─ utils.rs — Common utilities.
└─ tests
   └─ exhaustive testing
```


### Credits

- [subway](https://github.com/libevm/subway)
- [mev-template-rs](https://github.com/DeGatchi/mev-template-rs)
- [flashloan-rs](https://github.com/whitenois3/flashloan-rs)
