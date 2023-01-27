#[derive(Default, Clone, Copy, Debug)]
pub struct Challenges<T = Challenges> {
    evm_word: T,
    keccak_input: T,
    lookup_input: T
}

impl Challenges {
    
}