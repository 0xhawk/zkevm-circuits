pub mod evm_types;
use halo2_proofs::{
    arithmetic::{Field as Halo2Field, FieldExt},
    halo2curves::{
        group::ff::PrimeField,
    },
};

pub trait Field: FieldExt + Halo2Field + PrimeField<Repr = [u8; 32]> {}
