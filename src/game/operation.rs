use super::paddle::Sides;

use sdl2::keyboard::Keycode;

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

    pub fn from_key_code(keycode: Keycode) -> Option<Self> {
        match keycode {
            Keycode::W => Some(Self::new(OperationTypes::Up, Sides::Left, 0)),
            Keycode::S => Some(Self::new(OperationTypes::Down, Sides::Left, 0)),
            Keycode::A => Some(Self::new(OperationTypes::Up, Sides::Left, 1)),
            Keycode::D => Some(Self::new(OperationTypes::Down, Sides::Left, 1)),
            Keycode::Up => Some(Self::new(OperationTypes::Up, Sides::Right, 0)),
            Keycode::Down => Some(Self::new(OperationTypes::Down, Sides::Right, 0)),
            Keycode::Left => Some(Self::new(OperationTypes::Up, Sides::Right, 1)),
            Keycode::Right => Some(Self::new(OperationTypes::Down, Sides::Right, 1)),
            _ => None,
        }
    }
}
