use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::str::FromStr;

pub enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = s
            .split(' ')
            .take(2)
            .collect_tuple()
            .ok_or(anyhow!("cannot parse line"))?;

        let value = value.parse()?;

        match name {
            "forward" => Ok(Instruction::Forward(value)),
            "up" => Ok(Instruction::Up(value)),
            "down" => Ok(Instruction::Down(value)),
            x => Err(anyhow!("Invalid instruction: {}", x)),
        }
    }
}
