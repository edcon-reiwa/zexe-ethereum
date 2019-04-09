class Chain(object):

    def __init__(self, operator):
        self.operator = operator
        self.blocks = {}
        self.parent_queue = {}
        self.child_block_interval = 4
        self.next_child_block = self.child_block_interval
        self.next_deposit_block = 1
