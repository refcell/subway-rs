<img align="right" width="150" height="150" top="100" src="./assets/bot.png">

# bot • [![ci](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml) ![license](https://img.shields.io/badge/License-MIT-green.svg?label=license)

A Highly Optimized Sandwich Bot Built with Pure Rust and Huff.

> **Note**
>
> Test in prod. Something, something Zuck, move fast, break things, lose all your ETH.

### Usage

Required Preparation:

1. Install [rust](https://www.rust-lang.org/tools/install) if it's not already installed.
2. Deploy a sandwich contract from the [contracts](../contracts/) directory, or re-use an existing one.
3. Create a `.env` file and set the required variables, referencing the below environment variables. (NOTE: we've added `.env` and `.env.prod` to the `.gitignore` file, so you don't accidentally commit your secrets to the repo. All other secret commits are on you. Exercise caution.)

> **Warning**
>
> Some providers do NOT support all the RPC methods used by the bot.
>
> For instance, it is not possible to use infura to listen to pending transactions because `eth_newPendingTransactionFilter` is not supported.

```ignore
RPC_URL=http://127.0.0.1:8545
RPC_URL_WSS=ws://127.0.0.1:8546
PRIVATE_KEY=0000000000000000000000000000000000000000000000000000000000000001
FLASHBOTS_AUTH_KEY=0000000000000000000000000000000000000000000000000000000000000002
SANDWICH_CONTRACT=0x0000000000000000000000000000000000000000
```

Then, you can simply run the bot with:

```bash
cargo run --bin subway --release
```

And you should be good to go!

**Benchmarks**

To run benchmarks, simply run:

```bash
cargo bench
```

### Blueprint

```txt
.
├─ src
│  ├─ lib.rs — Exported modules with a re-exported prelude.
│  ├─ main.rs — The main bot binary.
│  ├─ numeric.rs — Refactored functions for numeric operations.
│  ├─ relayer.rs — Wrappers for network requests.
│  ├─ telemetry.rs — Telemetry for verbose logging.
│  ├─ uniswap.rs — Uniswap library.
│  └─ utils.rs — Common utilities.
└─ tests
   └─ Tests so exhaustive, it'll knock your (uni)-socks off
```

### Credits

- [subway](https://github.com/libevm/subway)
- [flashloan-rs](https://github.com/whitenois3/flashloan-rs)
