from plasma_core.transaction import Transaction


class Block(object):
    def __init__(self, number=0):
        self.number = number
        self.txs = {}
        self.root = 0

    def add(self, tx, root):
        self.txs = tx
        self.root = root
