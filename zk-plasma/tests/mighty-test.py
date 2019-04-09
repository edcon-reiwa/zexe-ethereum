import requests


# old serial number1: 0x729c4b45a77c37721e6c71d4d4e89b7c4b8a148745614c0986916fa73ae5236a

# old serial number2: 0x3802e45ca398f7566ddfde06ae68ad9a76048393bbd485a4c8048933df98b062

# new_commitment1: 0x8a9e673fce9165db7bf852d72a8101e7e5fe95478fcfb0cabd80be572bfa6e05

# new_commitment2: 0xbe9dfcbd43f22a7f30df085cccf2b44867c2a533305f016cbff0c29d1173400e

# ledger digest: 0x181d852efe62b47f27ce0feca00a2886e250d4b3918ba238f3c2ffe9ea4df605

# predicate commitment: 0x1a3252157780cd02b513ed2c107491e5ec6027079324bcb649158b02b1edb19a

# local data commitment: 0xbbb6b5c213ae7321327c55a622590346d1a5888690058dd4c015752b22491503

transaction = {
    "serialNumbers": [357654356874326504350, 5607435604375604325432],
    "newRecords": [321321321321, 57432530672504325],
    "memo": [1321321321, 12321321321],
    "in_root": 31321321,
    "in_proof": 21321321321
}


r = requests.post("http://localhost:8546/transfer", json=transaction)
assert(r.status_code == 200)
