use axum::http::StatusCode;
use uuid::Uuid;

use super::*;

#[sqlx::test]
async fn everything_works(pool: sqlx::PgPool) -> Result<(), (StatusCode, String)> {
    // list works on empty
    let users = list(&pool, UserFilter::default()).await?;
    assert_eq!(users.len(), 0);

    let bob = create(&pool, UserDeclaration::new("bob", "bob@example.com", "pw")).await?;
    let alice = create(&pool, UserDeclaration::new("alice", "alice@example.com", "pw")).await?;
    assert_eq!(bob.username, "bob");
    assert_eq!(alice.username, "alice");
    assert_ne!(bob.user_id, alice.user_id);

    // trim
    let john = create(&pool, UserDeclaration::new("john ", "john@example.com", "pw")).await?;
    assert_eq!(john.username, "john");

    // existing username
    assert!(create(&pool, UserDeclaration::new("bob", "robert@example.com", "pw")).await.is_err());
    assert!(create(&pool, UserDeclaration::new("bob ", "robert@example.com", "pw")).await.is_err());
    assert!(create(&pool, UserDeclaration::new("Bob", "robert@example.com", "pw")).await.is_err());

    // existing email
    assert!(create(&pool, UserDeclaration::new("robert", "bob@example.com", "pw")).await.is_err());
    assert!(create(&pool, UserDeclaration::new("bob", "bob+2@example.com", "pw")).await.is_err());
    assert!(create(&pool, UserDeclaration::new("robert", "bob+2@example.com", "pw")).await.is_ok());

    // invalid username
    assert!(create(&pool, UserDeclaration::new("John Doe", "doe@exampe.com", "pw")).await.is_err());
    assert!(create(&pool, UserDeclaration::new("JohnDoe", "doe@example.com", "pw")).await.is_ok());

    // describe
    assert!(describe(&pool, Uuid::new_v4()).await?.is_none());
    let re_bob = describe(&pool, bob.user_id).await?.unwrap();
    assert_eq!(bob, re_bob);

    // amend
    let bobby = amend(&pool, bob.user_id, UserAmendment { username: Some("bobby".into()) }).await?;
    assert_eq!(bobby.user_id, bob.user_id);
    assert_eq!(bobby.username, "bobby");
    let re_bobby = describe(&pool, bob.user_id).await?.unwrap();
    assert_eq!(bobby, re_bobby);
    let re_alice = describe(&pool, alice.user_id).await?.unwrap();
    assert_eq!(re_alice.username, "alice");

    // amend but nothing to change
    assert!(amend(&pool, bob.user_id, UserAmendment::default()).await.is_ok());

    // destroy
    destroy(&pool, bob.user_id).await?;
    assert!(describe(&pool, bob.user_id).await?.is_none());
    assert!(describe(&pool, alice.user_id).await?.is_some());

    // destroy non-existing user
    assert!(destroy(&pool, Uuid::new_v4()).await.is_ok());

    // list
    let users = list(&pool, UserFilter::default()).await?;
    assert_eq!(users.len(), 4);

    // list with a filter
    let users = list(&pool, UserFilter { username: Some("joHndOe".into()) }).await?;
    assert_eq!(users.len(), 1);
    let users = list(&pool, UserFilter { username: Some("alice".into()) }).await?;
    assert_eq!(users.len(), 1);
    let users = list(&pool, UserFilter { username: Some("elvis".into()) }).await?;
    assert_eq!(users.len(), 0);

    Ok(())
}
