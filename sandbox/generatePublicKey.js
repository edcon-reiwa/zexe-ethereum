var Wallet = require('ethereumjs-wallet');
var EthUtil = require('ethereumjs-util');

// Get a wallet instance from a private key
const privateKeyBuffer = EthUtil.toBuffer('0xf7a9c187e8ff9c25ee8841990118ff23af4c4fd5cd4d03d5e1457482cbe91b6d');
const wallet = Wallet.fromPrivateKey(privateKeyBuffer);

// Get a public key
const publicKey = wallet.getPublicKeyString();                                                                                                                                                                                                                                                               
console.log(wallet.getPublicKeyString());
console.log(wallet.getChecksumAddressString());
