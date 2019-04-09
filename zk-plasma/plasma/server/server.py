from plasma.child.child_chain import ChildChain
from plasma.root.root_chain import RootChain
import json
from flask import Flask
from flask import request
app = Flask(__name__)


print("INIT ROOT CHAIN")
zk_root = RootChain()

print("INIT CHILD CHAIN")
zk_child = ChildChain(0, zk_root)


@app.route('/transfer', methods=['POST'])
def transfer():
    if request.method == "POST":
        transaction = request.json

        print("Adding transaction to ledger", transaction)

        zk_child.apply_transaction(transaction)

    return "transaction included"
