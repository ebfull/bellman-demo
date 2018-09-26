#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;
use pairing::{Engine, Field};
use bellman::{Circuit, ConstraintSystem, SynthesisError};

struct DemoCircuit<'a, E: Engine> {
    _marker: std::marker::PhantomData<&'a E>,
}

// Implementation of our circuit.
impl<'a, E: Engine> Circuit<E> for DemoCircuit<'a, E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        Ok(())
    }
}


// Create some parameters, create a proof, and verify the proof.
fn main() {
    use pairing::bls12_381::Bls12;
    use std::marker::PhantomData;
    use rand::thread_rng;

    use bellman::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    };

    let rng = &mut thread_rng();

    let params = {
        let c = DemoCircuit::<Bls12> {
            _marker: PhantomData
        };

        generate_random_parameters(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    let c = DemoCircuit {
        _marker: PhantomData
    };

    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();

    assert!(verify_proof(&pvk, &proof, &[]).unwrap());
}
