<img align="right" width="150" height="150" top="100" src="./assets/subway.png">

# subway-rs • [![ci](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml) ![license](https://img.shields.io/badge/License-MIT-green.svg?label=license) ![twitter](https://img.shields.io/twitter/follow/asnared?style=social)

A practical demonstration of constructing sandwich attacks with a modern stack.


### What is this?

[subway-rs](https://github.com/abigger87/subway-rs) is a port of [libevm](https://twitter.com/libevm)'s original [subway](https://github.com/libevm/subway), implemented with [ethers-rs](https://github.com/gakonst/ethers-rs) and [huff](https://github.com/huff-language).

> Having highly optimized contracts is just one part of the equation, a tech stack is just as important as the contracts to execute on the opportunities.
_Source: [libevm/subway](https://github.com/libevm/subway#subway)_

If having a tech stack is just as important as optimized, secure contracts, then why not use the best language available for speed, dependability, and scalability?

This is Hugo: A pure-rust bot used to execute sandwich attacks on UniswapV2.

Hugo is **fast**. But don't take our word for it, just check out the [benchmarks](./hugo/benches).

Alongside Hugo, we have published [subway-rs](https://crate.io/crates/subway-rs): generalized, modular rust infrastructure that you may extend for your own MEV operations!

Hugo's goal is to act as a low barrier of entry for rust-based MEV development - Reference source code for aspiring new searchers.

This bot contains:

- read from the mempool
- decode transaction data
- simple logging system
- profit calculation algos
- gas bribe calculation
- bundle firing
- misc
  - doing math in JS
  - calculating next base fee

While the bot is functional, the bot logic is a very simplistic one and does not contain a lot of the features that many advance searchers have (but not including), such as:

- circuit breakers
- poison token checker
- caching system
- robust logging system (e.g. graphana)
- various gas saving ALPHAs

As such, this bot is intended as a piece of educational content, and not for production use.


### Blueprint

```ml
.
├─ bot — A highly optimized, pure rust sandwich bot.
|  └─ ...
└─ contracts — UniswapV2 sandwich attack contracts.
   └─ ...
```


### Acknowledgements

- [subway](https://github.com/libevm/subway)
- [foundry](https://github.com/foundry-rs/foundry)
- [forge-std](https://github.com/brockelmore/forge-std)
- [foundry-huff](https://github.com/foundry-rs/foundry-huff)


### Contributing

All contributions are welcome!

Please reach out to [asnared](https://twitter.com/asnared) on twitter if you have any questions.