# zexe-ethereum
An implementation of ZEXE on Ethereum

![sequence](https://raw.githubusercontent.com/edcon-reiwa/zexe-ethereum/master/docs/zexe_sequence.png)

## Setup
Install rust:
```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```
then,
```
cargo build --release
```

## Generate Tx components
```
./target/release/zexe-eth gen-tx
```
