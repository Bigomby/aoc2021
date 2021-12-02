use super::{engine::Engine, instruction::Instruction};
use std::marker::PhantomData;

pub type Position = (i64, i64, i64);

pub struct Submarine<E> {
    pub x: i64,
    pub y: i64,
    pub aim: i64,
    phantom: PhantomData<E>,
}

impl<E: Engine> Submarine<E> {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            aim: 0,
            phantom: PhantomData,
        }
    }

    pub fn move_position(&mut self, instruction: Instruction) {
        let current_position = (self.x, self.y, self.aim);
        let new_position = E::compute_position(current_position, instruction);

        self.x = new_position.0;
        self.y = new_position.1;
        self.aim = new_position.2;
    }
}
