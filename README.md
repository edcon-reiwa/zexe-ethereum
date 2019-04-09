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