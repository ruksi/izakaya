use crate::prelude::*;
use crate::user::model::User;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UserFilter {
    pub username: Option<String>,
}

pub async fn list(db: &sqlx::PgPool, filter: UserFilter) -> Result<Vec<User>> {
    let mut query = sqlx::QueryBuilder::new(
        // language=SQL
        r#"select user_id, username from "user""#,
    );

    // TODO: maybe ilike?
    if filter != UserFilter::default() {
        query.push(" where");
        let mut conditions = query.separated(" and");
        if let Some(username) = filter.username {
            conditions
                .push(" username = ")
                .push_bind_unseparated(username);
        }
    }

    let users: Vec<User> = query.build_query_as().fetch_all(db).await?;
    Ok(users)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::user::model::{create, UserDeclaration};

    use super::*;

    #[sqlx::test]
    async fn list_works(pool: sqlx::PgPool) -> Result<()> {
        let bob_filter = UserFilter {
            username: Some("bob".into()),
        };
        let alice_filter = UserFilter {
            username: Some("alice".into()),
        };
        let john_filter = UserFilter {
            username: Some("JoHn".into()),
        };

        assert_eq!(list(&pool, UserFilter::default()).await?.len(), 0);
        assert_eq!(list(&pool, bob_filter.clone()).await?.len(), 0);
        assert_eq!(list(&pool, alice_filter.clone()).await?.len(), 0);
        assert_eq!(list(&pool, john_filter.clone()).await?.len(), 0);

        let declaration = UserDeclaration::new("bob", "bob@example.com", "pw");
        create(&pool, declaration).await?;

        assert_eq!(list(&pool, UserFilter::default()).await?.len(), 1);
        assert_eq!(list(&pool, bob_filter.clone()).await?.len(), 1);
        assert_eq!(list(&pool, alice_filter.clone()).await?.len(), 0);
        assert_eq!(list(&pool, john_filter.clone()).await?.len(), 0);

        let declaration = UserDeclaration::new("alice", "alice@example.com", "pw");
        create(&pool, declaration).await?;

        assert_eq!(list(&pool, UserFilter::default()).await?.len(), 2);
        assert_eq!(list(&pool, bob_filter.clone()).await?.len(), 1);
        assert_eq!(list(&pool, alice_filter.clone()).await?.len(), 1);
        assert_eq!(list(&pool, john_filter.clone()).await?.len(), 0);

        let declaration = UserDeclaration::new("john ", "john@example.com", "pw");
        create(&pool, declaration).await?;

        assert_eq!(list(&pool, UserFilter::default()).await?.len(), 3);
        assert_eq!(list(&pool, bob_filter.clone()).await?.len(), 1);
        assert_eq!(list(&pool, alice_filter.clone()).await?.len(), 1);
        assert_eq!(list(&pool, john_filter.clone()).await?.len(), 1);

        Ok(())
    }
}
