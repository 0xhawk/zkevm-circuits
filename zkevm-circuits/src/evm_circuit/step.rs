use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{ConstraintSystem},
};

#[derive(Clone, Copy, Debug)]
pub enum ExecutionState {
    ADD_SUB,
}

#[derive(Clone, Debug)]
pub(crate) struct Step<const F: usize> {
    pub(crate) state: StepState<F>,
}

impl<const F: usize: FieldExt> Step<const F: usize> {
    pub(crate) fn new(
        meta: &mut ConstraintSystem<F>,
    ) {
        meta;
    }
}

#[derive(Clone, Debug)]
pub(crate) struct StepState<const F: usize> {
    /// The execution state selector for the step
    pub(crate) execution_state: DynamicSelectorHalf<F>,
}