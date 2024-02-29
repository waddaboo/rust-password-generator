pub fn password_length_validator(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(n) if (8..101).contains(&n) => Ok(n),
        Ok(_) => Err("The number must be in between 8 to 100".to_string()),
        Err(_) => Err("Value must be an integer".to_string()),
    }
}

pub fn pin_length_validator(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(n) if (4..13).contains(&n) => Ok(n),
        Ok(_) => Err("The number must be in between 4 to 12".to_string()),
        Err(_) => Err("Value must be an integer".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{password_length_validator, pin_length_validator};

    #[test]
    fn test_password_length_validator() {
        assert!(password_length_validator("7").is_err());
        assert!(password_length_validator("8").is_ok());
        assert!(password_length_validator("99").is_ok());
        assert!(password_length_validator("101").is_err());
        assert!(password_length_validator("test").is_err());
    }

    #[test]
    fn test_pin_length_validator() {
        assert!(pin_length_validator("3").is_err());
        assert!(pin_length_validator("4").is_ok());
        assert!(pin_length_validator("10").is_ok());
        assert!(pin_length_validator("13").is_err());
        assert!(pin_length_validator("test").is_err());
    }
}
