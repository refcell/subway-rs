<img align="right" width="150" height="150" top="100" src="./assets/subway.png">

# subway-rs • [![ci](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml) ![license](https://img.shields.io/github/license/abigger87/subway-rs?label=license) ![solidity](https://img.shields.io/badge/solidity-^0.8.15-lightgrey) ![huff](https://img.shields.io/badge/huff-0.3.0-8b6c5c)

A practical demonstration of constructing sandwich attacks with a modern stack.


### What is this?

[subway-rs](https://github.com/abigger87/subway-rs) is a port of [libevm](https://twitter.com/libevm)'s original [subway](https://github.com/libevm/subway), implemented with [ethers-rs](https://github.com/gakonst/ethers-rs) and [huff](https://github.com/huff-language).

> Having highly optimized contracts is just one part of the equation, a tech stack is just as important as the contracts to execute on the opportunities.
_Source: [libevm/subway](https://github.com/libevm/subway#subway)_

Since Having 


### Blueprint

```ml
.
├─ bot — A highly optimized sandwich bot and related infrastructure written in pure rust.
|  └─ ...
└─ contracts — Contains contracts that can be used for front and back slices in a UniswapV2 sandwich attack.
   └─ ...
```


### Acknowledgements

- [subway](https://github.com/libevm/subway)
- [foundry](https://github.com/foundry-rs/foundry)
- [forge-std](https://github.com/brockelmore/forge-std)
- [foundry-huff](https://github.com/foundry-rs/foundry-huff)

