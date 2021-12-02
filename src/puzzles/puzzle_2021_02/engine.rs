use super::instruction::Instruction;
use super::submarine::Position;

pub trait Engine {
    fn compute_position(position: Position, instruction: Instruction) -> Position;
}
