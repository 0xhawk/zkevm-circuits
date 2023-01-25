use crate::{
    evm_circuit::{
        step::{ExecutionState},
        util::{
            constraint_builder::{ConstraintBuilder},
        },
    }
}

mod add_sub;

use add_sub::AddSubGadget;

pub(crate) trait ExecutionGadget<F: FieldExt> {
    const NAME: &'static str;

    const EXECUTION_STATE: ExecutionState;

    fn configure(cb: &mut ConstraintBuilder<F>) -> Self;

}

#[derive(Clone, Debug)]
pub(crate) struct ExecutionConfig<F> {

}

impl<F: Field> ExecutionConfig<F> {

}