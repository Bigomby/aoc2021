use itertools::Itertools;

pub enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl Instruction {
    pub fn parse(input: &str) -> Self {
        let (name, value) = input.split(' ').take(2).collect_tuple().unwrap();
        let value = value.parse().unwrap();

        match name {
            "forward" => Instruction::Forward(value),
            "up" => Instruction::Up(value),
            "down" => Instruction::Down(value),
            _ => unimplemented!(),
        }
    }
}
