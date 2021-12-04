use std::str::FromStr;

pub struct PuzzleInput {
    content: String,
}

impl PuzzleInput {
    pub fn new<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn lines<T: FromStr>(&self) -> Result<Vec<T>, <T as FromStr>::Err> {
        self.content.lines().map(T::from_str).collect()
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64() {
        let content = "1\n2\n3";
        let input = PuzzleInput::new(content);

        let result: Vec<i64> = input.lines().unwrap();

        assert_eq!(result, [1, 2, 3]);
    }
}
