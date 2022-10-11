<img align="right" width="150" height="150" top="100" src="./assets/readme.jpg">

# subway-rs • [![ci](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/abigger87/subway-rs/actions/workflows/ci.yaml) ![license](https://img.shields.io/github/license/abigger87/subway-rs?label=license) ![solidity](https://img.shields.io/badge/solidity-^0.8.15-lightgrey) ![huff](https://img.shields.io/badge/huff-0.3.0-8b6c5c)

A practical demonstration of constructing sandwich attacks with a modern stack.


### What is this?

[subway-rs](https://github.com/abigger87/subway-rs) is a port of [libevm]()'s original [subway](https://github.com/libevm/subway), implemented with [ethers-rs](https://github.com/gakonst/ethers-rs) and [huff](https://github.com/huff-language).




### Blueprint

```ml
lib
├─ forge-std — https://github.com/foundry-rs/forge-std
├─ solmate — https://github.com/Rari-Capital/solmate
scripts
├─ Deploy.s.sol — Simple Deployment Script
src
├─ Greeter — A Minimal Greeter Contract
test
└─ Greeter.t — Exhaustive Tests
```


### Acknowledgements

- [femplate](https://github.com/abigger87/femplate)
- [foundry](https://github.com/foundry-rs/foundry)
- [solmate](https://github.com/Rari-Capital/solmate)
- [forge-std](https://github.com/brockelmore/forge-std)
- [forge-template](https://github.com/foundry-rs/forge-template)
- [foundry-toolchain](https://github.com/foundry-rs/foundry-toolchain)

