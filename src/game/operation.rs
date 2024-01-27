use super::paddle::Sides;

#[derive(Debug, Clone, Copy)]
pub enum OperationTypes {
    Up,
    Down,
    Stay,
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub op_type: OperationTypes,
    pub side: Sides,
    pub index: usize,
}

impl Operation {
    pub fn new(op_type: OperationTypes, side: Sides, index: usize) -> Self {
        Self {
            op_type,
            side,
            index,
        }
    }
}
