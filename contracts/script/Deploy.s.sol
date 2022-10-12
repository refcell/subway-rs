// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import "foundry-huff/HuffDeployer.sol";
import "forge-std/Script.sol";

interface Sandwich {
    function recoverERC20(address token) external;
}

contract Deploy is Script {

    // TODO: parameterize this
    address constant USER = address(0xBEEFBABE);

    function run() public returns (Sandwich sandwich) {
        sandwich = Sandwich(
            HuffDeployer
                .config()
                .with_addr_constant("USER", USER)
                .deploy("Sandwich")
        );
    }
}