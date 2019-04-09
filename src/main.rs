use clap::{Arg, App, SubCommand, AppSettings};
use std::fs::File;
use std::path::Path;
use std::string::String;
use rand::{SeedableRng, XorShiftRng};
use primitives::hexdisplay::{HexDisplay, AsBytesRef};

pub mod instantiated;
use instantiated::*;
use ::dpc::{
    dpc::plain_dpc::*,
    dpc::{DPCScheme, Record},
    crypto_primitives::*,
    ledger::*,
};
use dpc::plain_dpc::{
    predicate_circuit::{PredicateLocalData, EmptyPredicateCircuit, MintPredicateCircuit, ConservePredicateCircuit},
    LocalData,
    predicate::PrivatePredInput,
    DPC
};
use algebra::{to_bytes, ToBytes};
use snark::gm17::PreparedVerifyingKey;

use byteorder::{WriteBytesExt, LittleEndian};

fn main() {
    cli().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });
}

fn cli() -> Result<(), String> {
    const VERIFICATION_KEY_PATH: &str = "verification.params";
    const PROVING_KEY_PATH: &str = "proving.params";
    const DEFAULT_AMOUNT: &str = "100";
    const DEFAULT_MODE: &str = "MINT";

    let matches = App::new("zexe-eth")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.1.0")
        .author("Osuke Sudo")
        .about("Zexe on ethereum")
        .subcommand(SubCommand::with_name("gen-tx")
            .about("Performs a trusted setup for a given constraint system and generate a transaction")
                .arg(Arg::with_name("proving-key-path")
                    .short("p")
                    .long("proving-key-path")
                    .help("Path of the generated proving key file")
                    .value_name("FILE")
                    .takes_value(true)
                    .required(false)
                    .default_value(PROVING_KEY_PATH)
                )
                .arg(Arg::with_name("verification-key-path")
                    .short("v")
                    .long("verification-key-path")
                    .help("Path of the generated verification key file")
                    .value_name("FILE")
                    .takes_value(true)
                    .required(false)
                    .default_value(VERIFICATION_KEY_PATH)
                )
                .arg(Arg::with_name("amount")
                    .short("a")
                    .long("amount")
                    .help("The minted or transferred amount")
                    .takes_value(true)
                    .required(false)
                    .default_value(DEFAULT_AMOUNT)
                )
                .arg(Arg::with_name("mode")
                    .short("m")
                    .long("mode")
                    .help("The minted or transferred mode")
                    .takes_value(true)
                    .required(false)
                    .default_value(DEFAULT_MODE)
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("gen-tx", Some(sub_matches)) => {
            println!("Peforming setup...");
            let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

            let ledger_parameters = MerkleTreeIdealLedger::setup(&mut rng).expect("Ledger setup failed");
            let parameters =
                <InstantiatedDPC as DPCScheme<MerkleTreeIdealLedger>>::setup(&ledger_parameters, &mut rng)

                    .expect("DPC setup failed");
            println!("Generating transaction...");
            #[cfg(debug_assertions)]
            let pred_nizk_pvk: PreparedVerifyingKey<_> = parameters.pred_nizk_pp.vk.clone().into();

            // Generate metadata and an address for a dummy initial, or "genesis", record.
            let genesis_metadata = [1u8; 32];
            let genesis_address =
                DPC::create_address_helper(&parameters.comm_and_crh_pp, &genesis_metadata, &mut rng)
                    .unwrap();

            let genesis_sn_nonce =
                SnNonceCRH::evaluate(&parameters.comm_and_crh_pp.sn_nonce_crh_pp, &[34u8; 1]).unwrap();

            let genesis_pred_vk_bytes = to_bytes![PredVkCRH::evaluate(
                &parameters.comm_and_crh_pp.pred_vk_crh_pp,
                &to_bytes![parameters.pred_nizk_pp.vk].unwrap()
            )
            .unwrap()]
            .unwrap();

            let genesis_record = DPC::generate_record(
                &parameters.comm_and_crh_pp,
                &genesis_sn_nonce,
                &genesis_address.public_key,
                true, // The inital record should be dummy
                &[2u8; 32],
                &Predicate::new(genesis_pred_vk_bytes.clone()),
                &Predicate::new(genesis_pred_vk_bytes.clone()),
                &mut rng,
            )
            .unwrap();

            // Generate serial number for the genesis record.
            let genesis_sn = DPC::generate_sn(&genesis_record, &genesis_address.secret_key).unwrap();
            let genesis_memo = [1u8; 32];

            // Use genesis record, serial number, and memo to initialize the ledger.
            let ledger = MerkleTreeIdealLedger::new(
                ledger_parameters,
                genesis_record.commitment(),
                genesis_sn.clone(),
                genesis_memo,
            );

            // Generate dummy input records having as address the genesis address.
            let old_asks = vec![genesis_address.secret_key.clone(); NUM_INPUT_RECORDS];
            // println!("alice secret key:{:?}", genesis_address.secret_key);

            let mut old_records = vec![];
            for i in 0..NUM_INPUT_RECORDS {
                let old_sn_nonce = SnNonceCRH::evaluate(
                    &parameters.comm_and_crh_pp.sn_nonce_crh_pp,
                    &[64u8 + (i as u8); 1],
                )
                .unwrap();
                let old_record = DPC::generate_record(
                    &parameters.comm_and_crh_pp,
                    &old_sn_nonce,
                    &genesis_address.public_key,
                    true, // The input record is dummy
                    &[2u8; 32],
                    &Predicate::new(genesis_pred_vk_bytes.clone()),
                    &Predicate::new(genesis_pred_vk_bytes.clone()),
                    &mut rng,
                )
                .unwrap();
                old_records.push(old_record);
            }

            let amount_str = sub_matches.value_of("amount").unwrap();
            let amount: u32 = amount_str.parse().unwrap();

            let mode_str = sub_matches.value_of("mode").unwrap();

            // Construct new records.

            // Create an address for an actual new record.
            let new_metadata = [2u8; 32];
            let new_address =
                DPC::create_address_helper(&parameters.comm_and_crh_pp, &new_metadata, &mut rng).unwrap();

            // Create a payload.
            let new_dummy_payload = [2u8; 32];

            // Create a minted payload
            let mut new_mint_payload = [0u8; 32];

            if mode_str == "MINT" {
                (&mut new_mint_payload[0..4]).write_u32::<LittleEndian>(amount).unwrap();
            }

            // Set the new records' predicate to be the "always-accept" predicate.
            let new_predicate = Predicate::new(genesis_pred_vk_bytes.clone());

            let new_apks = vec![new_address.public_key.clone(); NUM_OUTPUT_RECORDS];
            let mut v = [0u8; 64];
            new_address.public_key.write(&mut v[..]).unwrap();
            println!("public key:{:?}", hex::encode(&v[..]));

            // let new_payloads = vec![new_payload.clone(); NUM_OUTPUT_RECORDS];
            let new_payloads = vec![new_mint_payload];
            let new_birth_predicates = vec![new_predicate.clone(); NUM_OUTPUT_RECORDS];
            let new_death_predicates = vec![new_predicate.clone(); NUM_OUTPUT_RECORDS];
            let new_dummy_flags = vec![false; NUM_OUTPUT_RECORDS];

            let auxiliary = [3u8; 32];
            let memo = [4u8; 32];

            let old_death_vk_and_proof_generator = |local_data: &LocalData<Components>| {
                let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
                let mut old_proof_and_vk = vec![];
                for i in 0..NUM_INPUT_RECORDS {
                    let proof = PredicateNIZK::prove(
                        &parameters.pred_nizk_pp.pk,
                        EmptyPredicateCircuit::new(
                            &local_data.comm_and_crh_pp,
                            &local_data.local_data_comm,
                            i as u8,
                        ),
                        &mut rng,
                    )
                    .expect("Proving should work");
                    #[cfg(debug_assertions)]
                    {
                        let pred_pub_input: PredicateLocalData<Components> = PredicateLocalData {
                            local_data_comm_pp: local_data.comm_and_crh_pp.local_data_comm_pp.clone(),
                            local_data_comm:    local_data.local_data_comm.clone(),
                            position:           i as u8,
                        };
                        assert!(
                            PredicateNIZK::verify(&pred_nizk_pvk, &pred_pub_input, &proof)
                                .expect("Proof should verify")
                        );
                    }

                    let private_input: PrivatePredInput<Components> = PrivatePredInput {
                        vk: parameters.pred_nizk_pp.vk.clone(),
                        proof,
                    };
                    old_proof_and_vk.push(private_input);
                }
                old_proof_and_vk
            };

            let new_birth_vk_and_proof_generator = |local_data: &LocalData<Components>| {
                let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
                let mut new_proof_and_vk = vec![];
                for i in 0..NUM_OUTPUT_RECORDS {
                    let proof = MintPredicateNIZK::prove(
                        &parameters.pred_nizk_pp.pk,
                        MintPredicateCircuit::new(
                            &local_data.comm_and_crh_pp,
                            &local_data.local_data_comm,
                            i as u8,
                            amount,
                        ),
                        &mut rng,
                    )
                    .expect("Proving should work");
                    #[cfg(debug_assertions)]
                    {
                        let pred_pub_input: PredicateLocalData<Components> = PredicateLocalData {
                            local_data_comm_pp: local_data.comm_and_crh_pp.local_data_comm_pp.clone(),
                            local_data_comm:    local_data.local_data_comm.clone(),
                            position:           i as u8,
                        };
                        assert!(
                            PredicateNIZK::verify(&pred_nizk_pvk, &pred_pub_input, &proof)
                                .expect("Proof should verify")
                        );
                    }
                    let private_input: PrivatePredInput<Components> = PrivatePredInput {
                        vk: parameters.pred_nizk_pp.vk.clone(),
                        proof,
                    };
                    new_proof_and_vk.push(private_input);
                }
                new_proof_and_vk
            };

            let (_new_records, transaction) = InstantiatedDPC::execute(
                &parameters,
                &old_records,
                &old_asks,
                &old_death_vk_and_proof_generator,
                &new_apks,
                &new_dummy_flags,
                &new_payloads,
                &new_birth_predicates,
                &new_death_predicates,
                &new_birth_vk_and_proof_generator,
                &auxiliary,
                &memo,
                &ledger,
                &mut rng,
            )
            .unwrap();

            assert!(InstantiatedDPC::verify(&parameters, &transaction, &ledger).unwrap());

            let mut old_serial_number1_v = [0u8; 32];
            let mut new_commitment1_v = [0u8; 32];
            let mut stuff_digest_v = [0u8; 32];
            // let stuff_core_proof_v: Vec<u8> = vec![];
            // let stuff_predicate_proof_v: Vec<u8> = vec![];
            let mut stuff_predicate_comm_v = [0u8; 32];
            let mut stuff_local_data_comm_v = [0u8; 32];

            transaction.old_serial_numbers[0].write(&mut old_serial_number1_v[..]).unwrap();

            transaction.new_commitments[0].write(&mut new_commitment1_v[..]).unwrap();

            transaction.stuff.digest.write(&mut stuff_digest_v[..]).unwrap();
            // transaction.stuff.core_proof.write(stuff_core_proof_v).unwrap();
            // transaction.stuff.predicate_proof.write(stuff_predicate_proof_v).unwrap();
            transaction.stuff.predicate_comm.write(&mut stuff_predicate_comm_v[..]).unwrap();
            transaction.stuff.local_data_comm.write(&mut stuff_local_data_comm_v[..]).unwrap();

            println!(
                "
                \nold serial number: 0x{}
                \nnew_commitment: 0x{}
                \nledger digest: 0x{}
                \npredicate commitment: 0x{}
                \nlocal data commitment: 0x{}
                ",
                // \ncore proof: 0x{}
                // \npredicate proof: 0x{}
                HexDisplay::from(&old_serial_number1_v as &AsBytesRef),
                HexDisplay::from(&new_commitment1_v as &AsBytesRef),
                HexDisplay::from(&stuff_digest_v as &AsBytesRef),
                // HexDisplay::from(&&stuff_core_proof_v[..] as &AsBytesRef),
                // HexDisplay::from(&&stuff_predicate_proof_v[..] as &AsBytesRef),
                HexDisplay::from(&stuff_predicate_comm_v as &AsBytesRef),
                HexDisplay::from(&stuff_local_data_comm_v as &AsBytesRef),
                );

        },
        _ => unreachable!()
    }
    Ok(())
}