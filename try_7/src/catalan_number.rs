use std::fmt::{Display, Formatter, Result};

pub struct CatalanNumber {
    bits: Vec<usize>,
}

impl Display for CatalanNumber {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        self.bits
            .iter()
            .rev()
            .try_for_each(|bit| fmt.write_fmt(format_args!("{bit}")))
    }
}

impl From<Vec<usize>> for CatalanNumber {
    fn from(bits: Vec<usize>) -> Self {
        Self { bits }
    }
}
