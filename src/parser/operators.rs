pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub(crate) fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}
