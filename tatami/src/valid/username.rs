use once_cell::sync::Lazy;
use regex::Regex;
use validator::ValidationError;

const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 32;

pub fn username(username: &str) -> Result<(), ValidationError> {
    if username.len() < MIN_LENGTH || username.len() > MAX_LENGTH {
        let mut err = ValidationError::new("length");
        err.add_param("min".into(), &MIN_LENGTH);
        err.add_param("max".into(), &MAX_LENGTH);
        return Err(err);
    }

    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9\-]+[a-zA-Z0-9]$").unwrap());
    if !RE.is_match(username) {
        let mut err = ValidationError::new("regex");
        err.message = Some("Username must be aLpHaNuMeR1c, but may contain hyphens (-)".into());
        return Err(err);
    }
    Ok(())
}
