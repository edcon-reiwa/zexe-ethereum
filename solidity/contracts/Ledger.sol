pragma solidity ^0.5.0;

import "./utils/MerkleTree.sol";
import "./utils/Verifier.sol";
import "./utils/MiMC.sol";


contract Ledger {
    
    using MerkleTree for MerkleTree.Data;

    // Stores the nullifiers for every spent serial number
    mapping (uint256 => bool) public nullifiers;

    // Stores all of the valid merkle tree roots
    mapping (uint256 => bool) public roots;

    // verifier key
    uint256[14] private m_vk;
    uint256[] private m_gammaABC;

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
        
        // TODO
        // Add check if record is dummy
        // If dummy then construct an instance for the snark proof

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

        // bool is_valid = VerifyProof(in_root, in_nullifier, GetExtHash(), in_proof);
        bool is_valid = VerifyProof(serialNumbers, newRecords, memo, in_root, in_proof);

        require( is_valid, "Proof invalid!" );

        for (uint256 i = 0; i < newRecords.length; i++) {
            Insert(newRecords[i]);
            emit TransferHash(newRecords[i], memo[i]);
        }
    }

    function VerifyProof(
        uint256[] memory serialNumbers,
        uint256[] memory newRecords,
        bytes32[] memory memo,
        uint256 in_root,
        uint256[8] memory proof
    )
        public view returns (bool) {
        // construct public input to the zkSNARK
        // public parameters: ledger digest st, old record serial numer sn,
        // new record commitments cm, transaction memorandum, memo
        uint256[] memory snark_input = new uint256[](1);
        snark_input[0] = HashPublicInputs(serialNumbers, newRecords, memo, in_root);

        // TODO
        // Retrieve verifying key
        uint256[14] memory vk;
        uint256[] memory vk_gammaABC;
        (vk, vk_gammaABC) = GetVerifyingKey();

        return true;
    }

    function HashPublicInputs(
        uint256[] memory serialNumbers,
        uint256[] memory newRecords,
        bytes32[] memory memo,
        uint256 in_root
    )
        public pure returns (uint256) {
        uint256 length = serialNumbers.length + newRecords.length + memo.length + 1;
        uint256[] memory inputs_to_hash = new uint256[](length);

        for (uint256 i = 0; i < serialNumbers.length; i++) {
            inputs_to_hash[i] = serialNumbers[i];            
        }

        uint256 current_length = serialNumbers.length;

        for (uint256 i = 0; i < newRecords.length; i++) {
            inputs_to_hash[i + current_length] = newRecords[i];            
        }
        
        current_length = serialNumbers.length + newRecords.length;

        for (uint256 i = 0; i < memo.length; i++) {
            inputs_to_hash[i + current_length] = uint256(memo[i]);            
        }

        inputs_to_hash[length - 1] = in_root;

        return MiMC.Hash(inputs_to_hash);
    }

    function GetVerifyingKey ()
        public view returns (uint256[14] memory out_vk, uint256[] memory out_gammaABC) {
            return (m_vk, m_gammaABC);
    }

    function SetVerifyingKey(uint256[14] memory in_vk, uint256[] memory in_gammaABC)
        public {
            m_vk = in_vk;
            m_gammaABC = in_gammaABC;
    }
}
