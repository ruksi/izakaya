use once_cell::sync::Lazy;
use regex::Regex;
use validator::ValidationError;

pub fn username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        let mut err = ValidationError::new("length");
        err.add_param("min".into(), &3);
        return Err(err);
    }

    static RE: Lazy<Regex> = Lazy::new(|| {
        return Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9\-]+[a-zA-Z0-9]$").unwrap();
    });
    if !RE.is_match(username) {
        let mut err = ValidationError::new("regex");
        err.message = Some("Username must be aLpHaNuMeR1c, but may contain hyphens (-)".into());
        return Err(err);
    }
    Ok(())
}
