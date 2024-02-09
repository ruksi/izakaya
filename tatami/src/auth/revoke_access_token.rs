use uuid::Uuid;

use crate::auth::{session_key, session_set_key};
use crate::prelude::*;
use crate::state::AppState;

pub async fn revoke_access_token(
    state: &AppState,
    access_token: String,
    user_id: Uuid,
) -> Result<()> {
    let mut cache_conn = state.cache_pool.get().await?;

    let session_key = session_key(access_token.clone());
    let session_list_key = session_set_key(user_id);

    redis::pipe()
        .srem(session_list_key, &session_key)
        .del(session_key)
        .query_async::<_, ()>(&mut cache_conn)
        .await?;

    Ok(())
}
