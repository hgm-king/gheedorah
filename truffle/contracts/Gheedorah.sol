pragma solidity ^0.5.0;


contract Gheedorah {

  string constant name = "Gheedorah";
  address owner_;

  event Purchase(string indexed storeId, string indexed client, uint credits);
  event Redeem(string indexed storeId, string indexed client, uint credits);

  mapping(string => mapping (string => uint256)) balances;

  using SafeMath for uint256;

  constructor() public {
    owner_ = msg.sender;
  }

  function balanceOf(string memory storeId, string memory clientId) public view returns (uint256) {
    return balances[storeId][clientId];
  }

  function credit(string memory storeId, string memory clientId, uint256 credits) public returns (bool) {
    require(msg.sender == owner_);
    balances[storeId][clientId] = balances[storeId][clientId].add(credits);

    emit Purchase(storeId, clientId, credits);
    return true;
  }

  function redeem(string memory storeId, string memory clientId, uint256 credits) public returns (bool) {
    require(msg.sender == owner_);
    require(credits <= balances[storeId][clientId]);

    balances[storeId][clientId] = balances[storeId][clientId].sub(credits);
    emit Redeem(storeId, clientId, credits);
    return true;
  }
}

library SafeMath {
  function sub(uint256 a, uint256 b) internal pure returns (uint256) {
    assert(b <= a);
    return a - b;
  }

  function add(uint256 a, uint256 b) internal pure returns (uint256) {
    uint256 c = a + b;
    assert(c >= a);
    return c;
  }
}
