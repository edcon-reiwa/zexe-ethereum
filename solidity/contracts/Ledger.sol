pragma solidity ^0.5.0;

import "./utils/MerkleTree.sol";
import "./utils/Verifier.sol";
import "./utils/MiMC.sol";


contract Ledger {
    
    using MerkleTree for MerkleTree.Data;

    // Stores the nullifiers for every spent serial number
    mapping (bytes32 => bool) public nullifiers;

    // Stores all of the valid merkle tree roots
    mapping (bytes32 => bool) public roots;

    // verifier key
    uint256[14] private m_vk;
    uint256[] private m_gammaABC;

    event TransferHash(bytes32 indexed newRecord, bytes32 indexed contentHash);

    MerkleTree.Data internal tree;

    constructor() public {
        // TODO FIX
        // adding initial mint record
        Insert(1);
    }

    function GetRoot()
        public view returns (uint256)
    {
        return tree.GetRoot();
    }

    function Insert(uint256 leaf) 
    internal returns (uint256 new_root, uint256 new_offset)
    {
        (new_root, new_offset) = tree.Insert(leaf);

        roots[bytes32(new_root)] = true;
    }

    function IsSpent(bytes32 nullifier)
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
        bytes32[] memory serialNumbers,
        bytes32[] memory newRecords,
        bytes32[] memory memo,
        bytes32 digest
        // uint256[8] memory in_proof TODO, re-add when proofs are working
    ) public returns(bool) {
        require(newRecords.length > 0, "newRecord list should not be empty");
        require(newRecords.length == memo.length, "Length of new records and memo must match");
        
        // TODO
        // Add check if record is dummy
        // If dummy then construct an instance for the snark proof

        require(serialNumbers.length > 0, "serial number should not be empty");
        require(roots[digest], "Must specify known merkle tree root");

        for (uint256 i = 0; i < serialNumbers.length; i++) {
            require( !nullifiers[serialNumbers[i]], "Cannot double-spend" );
            nullifiers[serialNumbers[i]] = true;
        }

        // TODO, re-add when proofs are working
        // bool is_valid = VerifyProof(serialNumbers, newRecords, memo, digest, in_proof);
        bool is_valid = VerifyProof(serialNumbers, newRecords, memo, digest);

        require(is_valid, "Proof invalid!");

        for (uint256 i = 0; i < newRecords.length; i++) {
            Insert(uint256(newRecords[i]));
            emit TransferHash(newRecords[i], memo[i]);
        }

        return true;
    }

    function VerifyProof(
        bytes32[] memory serialNumbers,
        bytes32[] memory newRecords,
        bytes32[] memory memo,
        bytes32 digest
        // uint256[8] memory proof TODO, re-add when proofs are working
    )
        public view returns (bool) {
        // construct public input to the zkSNARK
        // public parameters: ledger digest st, old record serial numer sn,
        // new record commitments cm, transaction memorandum, memo
        uint256[] memory snark_input = new uint256[](1);
        snark_input[0] = HashPublicInputs(serialNumbers, newRecords, memo, digest);

        // Retrieve verifying key
        uint256[14] memory vk;
        uint256[] memory vk_gammaABC;
        (vk, vk_gammaABC) = GetVerifyingKey();

        // call the snark verifier function
        // return Verifier.Verify(vk, vk_gammaABC, proof, snark_input);
        return true;
    }

    function HashPublicInputs(
        bytes32[] memory serialNumbers,
        bytes32[] memory newRecords,
        bytes32[] memory memo,
        bytes32 digest
    )
        public pure returns (uint256) {
        uint256 length = serialNumbers.length + newRecords.length + memo.length + 1;
        uint256[] memory inputs_to_hash = new uint256[](length);

        for (uint256 i = 0; i < serialNumbers.length; i++) {
            inputs_to_hash[i] = uint256(serialNumbers[i]);            
        }

        uint256 current_length = serialNumbers.length;

        for (uint256 i = 0; i < newRecords.length; i++) {
            inputs_to_hash[i + current_length] = uint256(newRecords[i]);            
        }
        
        current_length = serialNumbers.length + newRecords.length;

        for (uint256 i = 0; i < memo.length; i++) {
            inputs_to_hash[i + current_length] = uint256(memo[i]);            
        }

        inputs_to_hash[length - 1] = uint256(digest);

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
