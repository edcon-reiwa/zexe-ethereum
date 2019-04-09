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

## Setup
Build the library
```
cargo +nightly build --release
```

## Tutorial
Generate transaction components for minting privacy-preserving coins
```
./target/release/zexe-eth gen-tx --mode MINT --amount 100
```

## zk-Plasma
Start a web3 instance like ganache-cli.

```
ganache-cli
```

Run the zk-plasma server with:

```
python ./zk-plasma/zk-plasma.py
```

This will start a localhost server on port `8546`. It will also deploy the plasma root contract to ganache.

You can send transactions via the endpoint:
```
localhost:8546/transfer
```

This expects a `POST` including a transaction as a json in the form of:
```
transaction = {
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
truffle migrate
```

Deploy the ENS contracts
```
cd ens
truffle migrate
```
Starting the web interface
```
cd client
npm run start
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