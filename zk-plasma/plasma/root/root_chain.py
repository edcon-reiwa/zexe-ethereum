from web3 import Web3, HTTPProvider
import json


class RootChain(object):
    def __init__(self):
        self.web3 = Web3(HTTPProvider('http://localhost:8545'))
        self.web3.eth.defaultAccount = self.web3.eth.accounts[0]
        with open("./Ledger.abi") as f:
            abi = json.load(f)
        with open("./Ledger.bin") as f:
            binary = f.read()
        deploy = self.web3.eth.contract(bytecode=binary, abi=abi)
        tx_hash = deploy.constructor().transact()
        tx_receipt = self.web3.eth.waitForTransactionReceipt(tx_hash)

        self.contract = self.web3.eth.contract(
            address=tx_receipt.contractAddress,
            abi=abi,
        )
        print("Root chain deployed at: ", tx_receipt.contractAddress)

    def submitBlock(self, operator, root):
        tx_hash = self.contract.functions.submitBlock(
            root).transact()
        tx_receipt = self.web3.eth.waitForTransactionReceipt(tx_hash)
        print("Submitted block root: ", root)
        print("Transaction hash: ", tx_receipt['transactionHash'])
