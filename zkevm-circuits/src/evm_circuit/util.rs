use crate::{
    util::{Challenges},
};
use halo2_proofs::{
    plonk::{Expression},
};

pub(crate) mod constraint_builder;

#[derive(Clone, Debug)]
pub(crate) struct RandomLinearCombination<F, const N: usize> {
    expression: Expression<F>,
}

impl<F: FieldExt, const N: usize> RandomLinearCombination<F, N> {
    const N_BYTES: usize = N;

    pub(crate) fn new(cells: [Cell<F>; N], power_of_randomness: &[Expression<F>]) -> Self {
        Self {
            expression: Self::random_linear_combine_expr(
                cells.clone().map(|cell| cell.expr()),
                power_of_randomness
            ),
            cells,
        }
    }
}