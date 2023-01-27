use crate::{
    evm_circuit::{
        step::{ExecutionState, Step},
        util::{RandomLinearCombination},
    },
    util::{Challenges},
};
use eth_types::Field;
use halo2_proofs::{
    plonk::{
        Expression::{self},
    },
};

pub(crate) struct ConstraintBuilder<'a, F> {
    pub(crate) curr: Step<const F: usize>,
    pub(crate) next: Step<const F: usize>,
    challenges: &'a Challenges<Expression<F>>,
    word_powers_of_randomness: &'a [Expression<F>; 31],
    execution_state: ExecutionState,
    in_next_step: bool,
}

impl<'a, F: Field> ConstraintBuilder<'a, F> {

    pub(crate) fn query_word_rlc<const N: usize>(&mut self) -> RandomLinearCombination<F, N> {
        RandomLinearCombination::<F, N>::new(self.query_bytes(), self.word_powers_of_randomness)
    }

    pub(crate) fn query_bytes<const N: usize>(&mut self) -> [Cell<F>; N] {
        self.query_bytes_dyn(N).try_info().unwrap()
    }

    pub(crate) fn query_bytes_dyn(&mut self, count: usize) -> Vec<Cell<F>> {
        self.query_cells(CellType::Lookup(Table::Byte), count)
    }

    pub(crate) fn query_cell(&mut self) -> Cell<F> {
        self.query_cell_with_type(CellType::StoragePhase1)
    }

    pub(crate) fn query_cell_phase2(&mut self) -> Cell<F> {
        self.query_cell_with_type(CellType::StoragePhase2)
    }

    pub(crate) fn query_copy_cell(&mut self) -> Cell<F> {
        self.query_cell_with_type(CellType::StoragePermutation)
    }

    pub(crate) fn query_cell_with_type(&mut self, cell_type: CellType) -> Cell<F> {
        self.query_cells(cell_type, 1).first().unwrap().clone()
    }

    pub(crate) fn query_bool_with_type(&mut self, cell_type: CellType) -> Cell<F> {
        let cell = self.query_cell_with_type(cell_type);
        // self.require_boolean("Constrain cell to be a bool", cell.expr());
        cell
    }

    fn query_cells(&mut self, cell_type: CellType, count: usize) -> Vec<Cell<F>> {
        if self.in_next_step {
            &mut self.next
        } else {
            &mut self.curr
        }
        .cell_manager
        .query_cells(cell_type, count)
    }
}