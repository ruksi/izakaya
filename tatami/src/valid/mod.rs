pub use password::password;
pub use username::username;
pub use valid::Valid;

mod password;
mod username;
mod valid;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use validator::Validate;

    #[tokio::test]
    async fn valid_scope() -> Result<()> {
        #[derive(Validate)]
        struct Person {
            #[validate(length(min = 1, max = 10))]
            name: String,
        }
        let person = Person {
            name: "John".to_string(),
        };
        let valid_person = Valid::new(person)?;

        // you can get a reference to the inner value, but immutable only
        assert_eq!(valid_person.inner_as_ref().name, "John");

        // you can consume the inner value
        let person2 = valid_person.into_inner();
        assert_eq!(person2.name, "John");

        // you can't skip the validation step, the inner struct is private,
        // e.g. the next won't compile:
        // Valid {
        //     inner: ValidInner(Person {
        //         name: "John".to_string(),
        //     }),
        // };

        Ok(())
    }
}
