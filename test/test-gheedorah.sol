pragma solidity >=0.4.25 <0.6.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../truffle/contracts/Gheedorah.sol";

contract TestGheedorah {
  function testInvalidStoreAndInvalidUser() public {
    Gheedorah gheedorah = new Gheedorah();

    string memory storeId = "Kings Ivy & Moss";
    string memory clientId = "Max Planck";

    uint256 expected = 0;

    Assert.equal(gheedorah.balanceOf(storeId, clientId), expected, "New user should have 0 credits initially");
  }

  function testAddValueToUser() public {
    Gheedorah gheedorah = new Gheedorah();

    string memory storeId = "Kings Ivy & Moss";
    string memory clientId = "Marie Curie";

    Assert.equal(gheedorah.credit(storeId, clientId, 100), true, "User should be able to receive credit");
    Assert.equal(gheedorah.balanceOf(storeId, clientId), 100, "Now user should have 100 credits");
  }

  function testRedeemingValueFromUser() public {
    Gheedorah gheedorah = new Gheedorah();

    string memory storeId = "Kings Ivy & Moss";
    string memory clientId = "Tycho Brahe";

    gheedorah.credit(storeId, clientId, 100);

    Assert.equal(gheedorah.redeem(storeId, clientId, 100), true, "User should be able to redeem credit");
    Assert.equal(gheedorah.balanceOf(storeId, clientId), 0, "Now user should have no credits");
  }

  // this im not too sure about tbh
  function testNonOwnerViewIsAllowed() public {
    bool r;
    Gheedorah gheedorah = Gheedorah(DeployedAddresses.Gheedorah());

    (r, ) = address(this).call(abi.encodePacked(gheedorah.balanceOf.selector));
    Assert.isFalse(r, "Any account can call the balance of function");
  }

  // this im not too sure about tbh
  function testNonOwnerTransactionNotAllowed() public {
    bool r;
    Gheedorah gheedorah = Gheedorah(DeployedAddresses.Gheedorah());

    (r, ) = address(gheedorah).call(abi.encodePacked(gheedorah.credit.selector));
    Assert.isFalse(r, "Owner of contract only can call the credit function");
  }

}
