use super::scene::Sides;

use sdl2::keyboard::Keycode;

/// There are 3 types of operations on each paddle in total, that are up, down and stay.
#[derive(Debug, Clone, Copy)]
pub enum OperationTypes {
    /// The up operation.
    Up,
    /// The down operation.
    Down,
    /// The stay operation.
    Stay,
}

/// An operation on the game should act on a paddle in the game, so we need to specify which side
/// the paddles is on, and which index the paddle has.
#[derive(Debug, Clone, Copy)]
pub struct Operation {
    /// The type of the operation, whether the operation is up, down or stay.
    pub op_type: OperationTypes,
    /// The operation has to act on a paddle. This field determines which side the paddle is on.
    pub side: Sides,
    /// The operation has to act on a paddle. This field determines the index of the paddle.
    pub index: usize,
}

impl Operation {
    /// Constructor of an operation.
    pub fn new(op_type: OperationTypes, side: Sides, index: usize) -> Self {
        Self {
            op_type,
            side,
            index,
        }
    }

    /// Converts a keycode into an operation. This implementation only supports default games. This
    /// is because if there are more than 2 paddles, there is no obvious way of mapping keyboard
    /// inputs to operations.
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
