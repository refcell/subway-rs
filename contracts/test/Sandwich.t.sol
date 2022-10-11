// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import "forge-std/Test.sol";
import { HuffDeployer } from "foundry-huff/HuffDeployer.sol";

import { IWETH } from "interfaces/IWETH.sol";
import { IERC20 } from "interfaces/IERC20.sol";
import {
    IUniswapV2Router02,
    IUniswapV2Factory,
    IUniswapV2Pair
} from "interfaces/IUniswapV2.sol";

interface ISandwich {
    function recoverERC20(address token) external;
}

contract SandwichTest is Test {
    ISandwich sandwich;

    IWETH weth = IWETH(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);
    IERC20 usdc = IERC20(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48);
    IUniswapV2Pair wethUsdcPair;

    IUniswapV2Router02 univ2Router =
        IUniswapV2Router02(0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D);
    IUniswapV2Factory univ2Factory =
        IUniswapV2Factory(0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f);

    // The Sandwich User
    address constant USER = address(0xBEEFBABE);

    /// @notice Set up the testing suite
    function setUp() public {
        // Deposit ether into WETH Contract
        weth.deposit{value: 10e18}();

        // Get the WETH<>USDC Pair
        wethUsdcPair = IUniswapV2Pair(
            univ2Factory.getPair(address(weth), address(usdc))
        );

        // Overwrite inlined constants using the huff compiler - essentially equivalent to an immutable
        sandwich = ISandwich(HuffDeployer
            .config()
            .with_addr_constant("USER", USER)
            .deploy("Sandwich")
        );

        // Transfer the weth to the sandwich contract
        weth.transfer(address(sandwich), 1e18);
    }

    function testReceive(address caller) public {
        vm.assume(caller != USER);
        vm.startPrank(caller);

        // Calling the "receive" (fallback) with no value should revert
        vm.expectRevert();
        address(sandwich).call(bytes(""));

        // The rando can call with any value
        vm.deal(caller, 1e18);
        (bool s, ) = address(sandwich).call{value: 1}(bytes(""));
        assertTrue(s);
        vm.stopPrank();
    }

    function testOnlyUserCanCallFallback(address caller, bytes memory some) public {
        vm.assume(caller != USER);
        vm.assume(keccak256(some) != keccak256(bytes("")));

        // Only the user should be able to call the fallback with data
        vm.startPrank(caller);
        vm.expectRevert();
        address(sandwich).call(some);
        vm.stopPrank();

        // The USER can call the fallback with data
        vm.startPrank(USER);
        vm.expectRevert("DISPATCH_ERROR");
        address(sandwich).call(abi.encodeWithSignature("recoverERC20()"));
        vm.stopPrank();
    }

    // TODO: Add assertion back once implemented!
    function testSandwichFrontslice() public {
        bytes memory payload = getSandwichPayload();
        vm.startPrank(USER);
        uint256 _before = gasleft();
        (bool s, ) = address(sandwich).call(payload);
        uint256 _after = gasleft();
        // assertTrue(s);
        console2.log("Gas used: ", (_before - _after));
        vm.stopPrank();
    }


    // Helper methods

    /// @notice Constructs a sandwich payload
    function getSandwichPayload() internal view returns (bytes memory payload) {
        address[] memory path = new address[](2);
        path[0] = address(weth);
        path[1] = address(usdc);

        // Get amounts out
        uint256 amountIn = 1e18;
        uint256 amountOut = univ2Router.getAmountsOut(amountIn, path)[1];
        uint8 tokenOutNo = address(usdc) < address(weth) ? 0 : 1;

        payload = abi.encodePacked(
            address(weth), // token we're giving
            address(wethUsdcPair), // univ2 pair
            uint128(amountIn), // amountIn
            uint128(amountOut), // amountOut
            tokenOutNo
        );
    }


}