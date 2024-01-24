use validator::ValidationError;

const MIN_LENGTH: usize = 8;
const MAX_LENGTH: usize = 128;

pub fn password(password: &str) -> Result<(), ValidationError> {
    if password.len() < MIN_LENGTH || password.len() > MAX_LENGTH {
        let mut err = ValidationError::new("length");
        err.add_param("min".into(), &MIN_LENGTH);
        err.add_param("max".into(), &MAX_LENGTH);
        return Err(err);
    }
    Ok(())
}
