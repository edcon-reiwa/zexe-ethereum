pragma solidity ^0.5.6;

contract Ledger {
    uint256 height = 0;
    mapping(uint256 => uint256) roots;


    event BlockSubmitted(
        uint256 blockNumber,
        uint256 blockRoot
    );

    function submitBlock(uint256 _blockRoot) public {
        roots[height] = _blockRoot;
        
        emit BlockSubmitted(height, _blockRoot);

        height++;
    }
}