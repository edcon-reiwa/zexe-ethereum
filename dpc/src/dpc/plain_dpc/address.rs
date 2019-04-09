use crate::{
    crypto_primitives::{CommitmentScheme, PRF},
    dpc::{plain_dpc::PlainDPCComponents, AddressKeyPair},
};
use algebra::bytes::ToBytes;
use std::io::{Result as IoResult, Write};

#[derive(Derivative)]
#[derivative(
    Default(bound = "C: PlainDPCComponents"),
    Clone(bound = "C: PlainDPCComponents"),
    Debug(bound = "C: PlainDPCComponents")
)]
pub struct AddressPublicKey<C: PlainDPCComponents> {
    pub public_key: <C::AddrC as CommitmentScheme>::Output,
}

impl<C: PlainDPCComponents> ToBytes for AddressPublicKey<C> {
    fn write<W: Write>(&self, writer: W) -> IoResult<()> {
        self.public_key.write(writer)
    }
}

#[derive(Derivative)]
#[derivative(
    Default(bound = "C: PlainDPCComponents"),
    Clone(bound = "C: PlainDPCComponents"),
    Debug(bound = "C: PlainDPCComponents")
)]
pub struct AddressSecretKey<C: PlainDPCComponents> {
    pub sk_prf:   <C::P as PRF>::Seed,
    pub metadata: [u8; 32],
    pub r_pk:     <C::AddrC as CommitmentScheme>::Randomness,
}

#[derive(Derivative)]
#[derivative(Clone(bound = "C: PlainDPCComponents"))]
pub struct AddressPair<C: PlainDPCComponents> {
    pub public_key: AddressPublicKey<C>,
    pub secret_key: AddressSecretKey<C>,
}

impl<C: PlainDPCComponents> AddressKeyPair for AddressPair<C> {
    type AddressSecretKey = AddressSecretKey<C>;
    type AddressPublicKey = AddressPublicKey<C>;
}

// use crate::{
//     crypto_primitives::PRF,
//     dpc::{plain_dpc::PlainDPCComponents, AddressKeyPair},
// };
// use algebra::bytes::ToBytes;
// use std::io::{Result as IoResult, Write};
// use hex_literal::{hex, hex_impl};

// // #[derive(Derivative)]
// // #[derivative(
// //     Default(bound = "C: PlainDPCComponents"),
// //     Clone(bound = "C: PlainDPCComponents"),
// //     Debug(bound = "C: PlainDPCComponents")
// // )]
// #[derive(Clone, Debug)]
// pub struct AddressPublicKey {
//     // pub public_key: <C::AddrC as CommitmentScheme>::Output,
//     pub public_key: [u8; 64],
//     // _phantom: PhantomData<C>,
// }

// impl ToBytes for AddressPublicKey {
//     fn write<W: Write>(&self, writer: W) -> IoResult<()> {
//         self.public_key.write(writer)
//     }
// }

// // #[derive(Derivative)]
// // #[derivative(
// //     Default(bound = "C: PlainDPCComponents"),
// //     Clone(bound = "C: PlainDPCComponents"),
// //     Debug(bound = "C: PlainDPCComponents")
// // )]
// #[derive(Default, Clone, Debug)]
// pub struct AddressSecretKey{
//     // pub sk_prf:   <C::P as PRF>::Seed,
//     // pub metadata: [u8; 32],
//     // pub r_pk:     <C::AddrC as CommitmentScheme>::Randomness,
//     pub secret_key: [u8; 32],
//     // _phantom: PhantomData<C>,
// }


// // #[derive(Derivative)]
// // #[derivative(Clone(bound = "C: PlainDPCComponents"))]
// #[derive(Clone)]
// pub struct AddressPair{
//     pub public_key: AddressPublicKey,
//     pub secret_key: AddressSecretKey,
// }

// impl AddressKeyPair for AddressPair<C> {
//     type AddressSecretKey = AddressSecretKey;
//     type AddressPublicKey = AddressPublicKey;
// }


// pub fn alice_keypair() -> AddressPair {
//     let alice_private_key_v: [u8; 32] = hex!("f7a9c187e8ff9c25ee8841990118ff23af4c4fd5cd4d03d5e1457482cbe91b6d");
//     let alice_public_key_v: [u8; 64] = hex!("37c24f539dab1f19a438890684cf7d31b04d26ac487af43effd536859b12d89e80d54c890e342c0c7dede3c450c004b0be841293b059cc8d076c710fa20e4226");

//     AddressPair {
//         public_key: AddressPublicKey{ public_key: alice_public_key_v },
//         secret_key: AddressSecretKey{ secret_key: alice_private_key_v },
//     }
// }

// pub fn bob_keypair() -> AddressPair {
//     let bob_private_key_v = hex!("a092b53d4f6902421a5440f246ae5a50d9b4a8cfb02a80a5873e15438b77be0b");
//     let bob_public_key_v = hex!("2015b69865e50ec55c3c0501187995501fc399e4ccbdd2544e1ca775cd7d7fe0144c73005ec019a6b520fe5ec5f26d7f18e4bee3ccfeb554a40078896521ef08");

//     AddressPair {
//         public_key: AddressPublicKey{ public_key: bob_public_key_v },
//         secret_key: AddressSecretKey{ secret_key: bob_private_key_v },
//     }
// }
