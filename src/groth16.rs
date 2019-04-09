use dpc::crypto_primitives::nizk::NIZK;
use algebra::PairingEngine;
// use snark::Circuit;
use bellman_ce::Circuit;
use algebra::utils::ToEngineFr;
// use crate::engine_fr::ToEngineFr1;
use std::marker::PhantomData;
use bellman_ce::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
    Parameters, PreparedVerifyingKey, Proof, VerifyingKey
};
use pairing_ce::Engine;
use rand::Rng;
use failure::Error;

pub struct Groth16<E: Engine, C: Circuit<E>, V: ToEngineFr<E> + ?Sized> {
    _engine: PhantomData<E>,
    _circuit: PhantomData<C>,
    _verifier_input: PhantomData<V>,
}

impl<E: Engine, C: Circuit<E>, V: ToEngineFr<E> + ?Sized> NIZK for Groth16<E, C, V> {
    type Circuit = C;
    type AssignedCircuit = C;
    type ProvingParameters = Parameters<E>;
    type VerificationParameters = VerifyingKey<E>;
    type PreparedVerificationParameters = PreparedVerifyingKey<E>;
    type VerifierInput = V;
    type Proof = Proof<E>;

    fn setup<R: Rng>(
        circuit: Self::Circuit,
        rng: &mut R,
    ) -> Result<
        (
            Self::ProvingParameters,
            Self::PreparedVerificationParameters,
        ),
        Error,
    > {
        // let nizk_time = timer_start!(|| "{Groth 2016}::Setup");
        let pp = generate_random_parameters::<E, Self::Circuit, R>(circuit, rng)?;
        let vk = prepare_verifying_key(&pp.vk);
        // timer_end!(nizk_time);
        Ok((pp, vk))
    }

    fn prove<R: Rng>(
        pp: &Self::ProvingParameters,
        input_and_witness: Self::AssignedCircuit,
        rng: &mut R,
    ) -> Result<Self::Proof, Error> {
        // let proof_time = timer_start!(|| "{Groth 2016}::Prove");
        let result = create_random_proof::<E, _, _>(input_and_witness, pp, rng)?;
        // timer_end!(proof_time);
        Ok(result)
    }

    fn verify(
        vk: &Self::PreparedVerificationParameters,
        input: &Self::VerifierInput,
        proof: &Self::Proof,
    ) -> Result<bool, Error> {
        // let verify_time = timer_start!(|| "{Groth 2016}::Verify");
        // let conversion_time = timer_start!(|| "Convert input to E::Fr");
        let input = input.to_engine_fr()?;
        // timer_end!(conversion_time);

        // let verification = timer_start!(|| format!("Verify proof w/ input len: {}", input.len()));
        let result = verify_proof(&vk, proof, &input)?;
        // timer_end!(verification);
        // timer_end!(verify_time);
        Ok(result)
    }
}