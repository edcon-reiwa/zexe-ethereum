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
Build a CLI
```
cargo +nightly build --release
```

## Generate Tx components
```
./target/release/zexe-eth gen-tx --mode MINT --amount 100
```
