
pub(crate) struct AddSubGadget<F> {

}

impl<F: Field> ExecutionGadget<F> for AddSubGadget<F> {
    const NAME: &'static str = "ADD_SUB";

    const EXECUTION_STATE: ExecutionState = ExecutionState::ADD_SUB;
}