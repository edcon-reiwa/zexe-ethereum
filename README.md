# zexe-ethereum
An implementation of ZEXE on Ethereum

![sequence](https://raw.githubusercontent.com/edcon-reiwa/zexe-ethereum/master/docs/zexe_sequence.png)

## Dependencies
Install rust
```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup install nightly
```

[Install IPFS](https://docs.ipfs.io/introduction/install/) and start it
```
ipfs daemon
```

## Usage
There are some steps

1. Setup a snark library
2. Generate a transaction and a proof
3. Send a transaction to Plasma
4. Register commitments to Root chain (Smart contract)

### Generate transaction components for minting privacy-preserving coins
#### Build a snark library
```
cargo +nightly build --release
```

#### Generate transactions, proofs and send private inputs to IPFS
This takes a while because it setups snark and generate proofs.
This uploads private inputs to IPFS (TODO: data encryption), then prints serial numbers, commitments and verification keys.
```
./target/release/zexe-eth gen-tx --mode MINT --amount 100
```

It prints standard output like:
```
Performing setup...
Generating transaction...
public key:"f3a572f43856518650e3106d1ae05ee752374bf0310d11a9ecaae61772366c0c"
- Successfully generated a record file to /tmp/record.json
- Successfully uploaded the record.json to IPFS
  Response message: {"Name":"record.json","Hash":"QmTmAMs6LUw3VAT3E2M3hkHULxQHNSzqEeMn5iLoQTyyHS","Size":"650"}


old serial number: 0x729c4b45a77c37721e6c71d4d4e89b7c4b8a148745614c0986916fa73ae5236a

new_commitment: 0xed01a5fa40a0e43ddf6ba43a2c62da93cc731d826f63b4e7b1cdefecfee31711

ledger digest: 0x181d852efe62b47f27ce0feca00a2886e250d4b3918ba238f3c2ffe9ea4df605

predicate commitment: 0x58b854e483d0d1e17193c9468929b6f2d40ea5d66ef8c8bee3c5e281d9c07375

local data commitment: 0x02fc067f83b3e2e395f116899859aad94cdeae070a3abdb927750ceabff09812
```


### zk-Plasma
Start a web3 instance like ganache-cli.
```
ganache-cli
```

Install dependencies (Python v3.7 required)
It'd be better to create virtualenv before `pip install`
```
cd zk-plasma

# ↓ optional for virtualenv
virtualenv ./venv
source venv/bin/activate
# ↑ optional

pip install -r requirements.txt
cd ..
```

Run the zk-plasma server with:
```
source zk-plasma/venv/bin/activate # optional
python ./zk-plasma/zk-plasma.py
```

This will start a localhost server on port `8546`. It will also deploy the plasma root contract to ganache.

You can send transactions via the endpoint:
```
localhost:8546/transfer
```

This expects a `POST` with request body that is a transaction as a json in the form of:
```
{
    "serialNumbers": [357654356874326504350, 5607435604375604325432],
    "newRecords": [321321321321, 57432530672504325],
    "memo": [1321321321, 12321321321],
    "in_root": 31321321,
    "in_proof": 21321321321
}
```

## Running the web interface locally
Deploy the Leger Smart Contract
```
cd soldity
npm install
npm run migrate
```

Deploy the ENS contracts (Python v2.7 required)
```
cd ens
npm install
npm run migrate
```

Starting the web interface (Python v2.7 required)
```
cd client
npm install
npm start
```

### Notes
##### Generating and executing transactions
- `Transaction Generation` section is currently not integrated and only a mock. Transaction parameters need to be generated using the rust scripts
- Transferring is still not implemented but shouldn't be that different from minting
- Mint transaction parameters should be copied from the rust script output
- Past transactions (commitments of the newly minted tokens) are saved in the browser's local storage
- Current balance is constructed from these transaction records
- Deleting local storage will result in losing access to past records

##### ENS Management
- The account used to deploy the ENS contracts becomes the owner of the `.eth` domain
- `.eth` can be configured to anything else in the migration script `ens/migrations/2_deploy_contracts.js`
- Ownership can be transferred
- Anyone can regsister a subdomain from the web interface
    - eg. If a user registers Address: `0x1234...`, ENS Domain: `alice`, `0x1234...` becomes the owner of `alice.eth`
- Typing `alice.eth` will automatically be converted to the registered address
    - Testable through the `To:` field in `Transaction Generation`

## Reference
- [Zexe: Enabling Decentralized Private Computation](https://eprint.iacr.org/2018/962.pdf)
- [scipr-lab/zexe](https://github.com/scipr-lab/zexe)