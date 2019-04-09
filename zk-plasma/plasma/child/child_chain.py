from ethsnarks.merkletree import MerkleTree
from ethsnarks.mimc import mimc_hash
from plasma_core.block import Block
from plasma_core.transaction import Transaction


class ChildChain(object):

    def __init__(self, operator, root_chain):
        self.operator = operator
        self.root_chain = root_chain
        self.chain = {}
        self.tree = MerkleTree(2)
        self.roots = {}
        self.nullifiers = {}
        self.height = 0
        self.current_block = Block()

    def apply_exit(self):
        # TODO
        return

    def apply_deposit(self):
        # TODO
        return

    def apply_transaction(self, transaction):
        tx = Transaction(
            transaction["serialNumbers"],
            transaction["newRecords"],
            transaction["memo"],
            transaction["in_root"],
            transaction["in_proof"]
        )

        assert(self.approve_transaction(
            tx.serialNumbers,
            tx.newRecords,
            tx.memo,
            tx.in_root,
            tx.in_root
        ))

        # insert into tree
        for i in range(len(tx.newRecords)):
            self.insert(tx.newRecords[i])
            print("Added new commitment: ", tx.newRecords[i])
            print("Added new memo: ", tx.memo[i])

        self.current_block.add(tx, self.get_root())

        # For now, 1 tx = 1 block!
        self.submit_block(self.current_block)

    def submit_block(self, block):
        self.chain[self.height] = block
        self.root_chain.submitBlock(self.operator, block.root)

        self.height += 1
        self.current_block = Block(number=self.height)

    def get_root(self):
        return self.tree.root

    def insert(self, leaf):
        # insert leaf to merkle tree
        self.tree.append(leaf)
        self.roots[self.get_root()] = True

    def is_spent(self, nullifier):
        # check if utxo is spent
        return self.nullifiers[nullifier]

    def approve_transaction(self, serialNumbers, newRecords, memo, in_root, in_proof):
        # verify transaction
        assert(len(newRecords) > 0)
        assert(len(newRecords) == len(memo))
        assert(len(serialNumbers) > 0)
        # assert(self.roots[in_root])

        is_valid = self.verify_proof(
            serialNumbers, newRecords, memo, in_root, in_proof)

        assert(is_valid)

        # check if record is dummy
        for sn in serialNumbers:
            # assert(self.nullifiers[sn])
            self.nullifiers[sn] = True

        return True

    def verify_proof(self, serialNumbers, newRecords, memo, in_root, in_proof):
        # construct SNARK input
        snark_input = self.hash_public_inputs(
            serialNumbers, newRecords, memo, in_root)

        # return verifier.verify(self.vk, self.vk_gammaABC, in_proof, snark_input)
        return True

    def hash_public_inputs(self, serialNumbers, newRecords, memo, in_root):
        inputs_to_hash = []

        for sn in serialNumbers:
            inputs_to_hash.append(int(sn))

        for commit in newRecords:
            inputs_to_hash.append(int(commit))

        for m in memo:
            inputs_to_hash.append(int(m))

        inputs_to_hash.append(int(in_root))

        return mimc_hash(inputs_to_hash)
