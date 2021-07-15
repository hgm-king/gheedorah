pragma solidity ^0.5.0;

import "truffle/Assert.sol";

contract TestBytesLib2 {
    function testThrowFunctions() public {
        bool r;

        // We're basically calling our contract externally with a raw call, forwarding all available gas, with
        // msg.data equal to the throwing function selector that we want to be sure throws and using only the boolean
        // value associated with the message call's success
        (r, ) = address(this).call(abi.encodePacked(this.IThrow1.selector));
        Assert.isFalse(r, "If this is true, something is broken!");

        (r, ) = address(this).call(abi.encodePacked(this.IThrow1.selector));
        Assert.isFalse(r, "What?! 1 is equal to 10?");
    }

    function IThrow1(uint256 s) public pure {
        require(s == 1, "I will throw");
    }
}
