use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::str::FromStr;

use crate::util::{Coords, Line};

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = &s.split("->").collect_vec();

        let start = entries
            .get(0)
            .ok_or(anyhow!("cannot parse start position"))?;
        let end = entries.get(1).ok_or(anyhow!("cannot parse end position"))?;

        let (x1, y1) = start
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (x2, y2) = end
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        let start = Coords { x: x1, y: y1 };
        let end = Coords { x: x2, y: y2 };

        // Not horizontal or vertical
        if x1 != x2 && y1 != y2 {
            // Not diagonal
            if (x1 - x2).abs() != (y1 - y2).abs() {
                return Ok(Self::Invalid);
            }

            return Ok(Self::Diagonal(start, end));
        }

        if x1 != x2 {
            return Ok(Self::Horizontal(start, end));
        }

        if y1 != y2 {
            return Ok(Self::Vertical(start, end));
        }

        Ok(Self::Point(start))
    }
}
