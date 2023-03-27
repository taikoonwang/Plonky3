use alloc::vec::Vec;
use hyperfield::field::Field;

pub trait AlgebraicHash<F: Field, const OUT_WIDTH: usize> {
    fn hash(&self, input: Vec<F>) -> [F; OUT_WIDTH];
}
