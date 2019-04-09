class Transaction(object):
    def __init__(self, serialNumbers, newRecords, memo, in_root, in_proof):
        self.serialNumbers = serialNumbers
        self.newRecords = newRecords
        self.memo = memo
        self.in_root = in_root
        self.in_proof = in_proof
