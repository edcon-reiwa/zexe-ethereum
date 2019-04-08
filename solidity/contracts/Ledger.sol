pragma solidity ^0.5.0;

import "./utils/MerkleTree.sol";
import "./utils/Verifier.sol";


contract Ledger {
    
    using MerkleTree for MerkleTree.Data;

    // Stores the nullifiers for every spent serial number
    mapping (uint256 => bool) public nullifiers;

    // Stores all of the valid merkle tree roots
    mapping (uint256 => bool) public roots;


    event TransferHash(uint256 indexed newRecord, bytes32 indexed contentHash);

    MerkleTree.Data internal tree;

    bool minted = false;

    function GetRoot()
        public view returns (uint256)
    {
        return tree.GetRoot();
    }

    function Insert(uint256 leaf)
        internal returns (uint256 new_root, uint256 new_offset)
    {
        (new_root, new_offset) = tree.Insert(leaf);

        roots[new_root] = true;
    }

    function IsSpent(uint256 nullifier)
        public view returns (bool)
    {
        return nullifiers[nullifier];
    }

    function GetExtHash()
        public view returns (uint256)
    {
        return uint256(sha256(
            abi.encodePacked(
                address(this),
                msg.sender
            ))) % Verifier.ScalarField();
    }

    function ApproveTransaction(
        uint256[] memory serialNumbers,
        uint256[] memory newRecords,
        bytes32[] memory memo,
        uint256 in_root,
        uint256 in_nullifier,
        uint256[8] memory in_proof
    ) public returns(bool) {
        require(newRecords.length > 0);
        require(newRecords.length == memo.length, "Length of new records and memo must match");

        // special case for initial mint
        if (!minted) {
            require(in_root == 0);
            require(serialNumbers.length == 0);
            minted = true;
        } else {
            require(serialNumbers.length > 0);
            require( roots[in_root], "Must specify known merkle tree root" );
        }

        for (uint256 i = 0; i < serialNumbers.length; i++) {
            require( !nullifiers[serialNumbers[i]], "Cannot double-spend" );
            nullifiers[serialNumbers[i]] = true;
        }

        bool is_valid = VerifyProof(in_root, in_nullifier, GetExtHash(), in_proof);

        require( is_valid, "Proof invalid!" );

        for (uint256 i = 0; i < newRecords.length; i++) {
            Insert(newRecords[i]);
            emit TransferHash(newRecords[i], memo[i]);
        }
    }

    function VerifyProof(
        uint256 in_root,
        uint256 in_nullifier,
        uint256 in_exthash,
        uint256[8] memory proof
    )
        public view returns (bool)
    {
        // TODO
        // Verify snarks proof
        return true;
    }
}
