use validator::ValidationError;

pub fn password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 || password.len() > 128 {
        let mut err = ValidationError::new("length");
        err.add_param("min".into(), &8);
        err.add_param("max".into(), &128);
        return Err(err);
    }
    Ok(())
}
