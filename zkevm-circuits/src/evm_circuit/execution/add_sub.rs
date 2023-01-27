use crate::{
    evm_circuit::{
        execution::ExecutionGadget,
        step::ExecutionState,
        util::{
            constraint_builder::{ConstraintBuilder},
        },
    },
};
use eth_types::Field;


#[derive(Clone, Debug)]
pub(crate) struct AddSubGadget<F> {

}

impl<F: Field> ExecutionGadget<F> for AddSubGadget<F> {
    const NAME: &'static str = "ADD_SUB";

    const EXECUTION_STATE: ExecutionState = ExecutionState::ADD_SUB;

//     // fn configure(cb: &mut ConstraintBuilder<F>) -> Self {
//     //     let opcode = cb.query_call();

//     //     let a = cb.query_word_rlc();
        

    // }
}