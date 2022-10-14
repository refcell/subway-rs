<img align="right" width="150" height="150" top="100" src="./assets/hugo.png">

# subway-rs/hugo • [![ci](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml) ![license](https://img.shields.io/badge/License-MIT-green.svg?label=license) ![crates.io](https://img.shields.io/crates/v/subway-rs)


A highly optimized sandwich bot and related infrastructure written in pure rust.


> **Note**
>
> Test in prod. Something, something Zuck, move fast, break things, lose all your ETH.


### What is a Hugo?

Hugo is a bot built in rust to sandwich attack Uniswap V2 pairs.

In every Uniswap V2 trade, the user (victim) will specify a minimum amount of output tokens they're willing to receive.

Hugo's (the sandwich bot's) job is to calculate how much of the output tokens they should buy (to push the price of the token up) to match the victim's minimum out requirement. This minimum out requirement on most cases will be 2%, but on extreme cases it can be as high as 20% on volatile pairs (such as the SHIBA-WETH pair during the craze).

Once Hugo has calculated the optimal number of tokens to buy, it'll wait for the victim to buy their tokens, and immediately sell to gain a profit.


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
cargo run --bin hugo --release
```

And you should be good to go!


### Blueprint

```ml
.
├─ src
│  └─ lib.rs — Exported modules with a re-exported prelude.
└─ tests
   └─ Tests so exhaustive, it'll knock your (uni)-socks off
```


### Credits

- [subway](https://github.com/libevm/subway)
- [flashloan-rs](https://github.com/whitenois3/flashloan-rs)
