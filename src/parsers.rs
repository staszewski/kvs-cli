use std::num::ParseIntError;

pub fn parse_users_input(input: String) -> Result<i32, ParseIntError> {
    let parsed_value: Result<i32, ParseIntError> = match input.trim().parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(e),
    };

    parsed_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsed_correctly() {
        let result = parse_users_input(String::from("3"));
        assert_eq!(result.unwrap(), 3);
    }
    // cant test it for errors correctly https://github.com/rust-lang/rust/issues/22639
}
