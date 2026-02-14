use sqlx::PgPool;
use uuid::Uuid;

use crate::models::message::{Message, DirectMessage};
use crate::error::Result;

pub async fn create_message(
    pool: &PgPool,
    room_id: Uuid,
    user_id: Uuid,
    content: &str,
) -> Result<Message> {
    let msg = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (room_id, user_id, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(room_id)
    .bind(user_id)
    .bind(content)
    .fetch_one(pool)
    .await?;
    Ok(msg)
}

pub async fn get_room_messages(
    pool: &PgPool,
    room_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<Message>> {
    Ok(sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE room_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(room_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?)
}

pub async fn get_message(pool: &PgPool, id: Uuid) -> Result<Option<Message>> {
    Ok(sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?)
}

pub async fn delete_message(pool: &PgPool, id: Uuid) -> Result<u64> {
    let result = sqlx::query("DELETE FROM messages WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

// ──────────────────── Direct Messages ─────────────────
pub async fn create_direct_message(
    pool: &PgPool,
    sender_id: Uuid,
    recipient_id: Uuid,
    content: &str,
) -> Result<DirectMessage> {
    let dm = sqlx::query_as::<_, DirectMessage>(
        r#"
        INSERT INTO direct_messages (sender_id, recipient_id, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(sender_id)
    .bind(recipient_id)
    .bind(content)
    .fetch_one(pool)
    .await?;
    Ok(dm)
}

pub async fn get_dm_history(
    pool: &PgPool,
    user_a: Uuid,
    user_b: Uuid,
    limit: i64,
) -> Result<Vec<DirectMessage>> {
    Ok(sqlx::query_as::<_, DirectMessage>(
        r#"
        SELECT * FROM direct_messages
        WHERE (sender_id = $1 AND recipient_id = $2)
           OR (sender_id = $2 AND recipient_id = $1)
        ORDER BY created_at DESC LIMIT $3
        "#,
    )
    .bind(user_a)
    .bind(user_b)
    .bind(limit)
    .fetch_all(pool)
    .await?)
}

pub async fn mark_dms_read(pool: &PgPool, sender_id: Uuid, recipient_id: Uuid) -> Result<()> {
    sqlx::query(
        "UPDATE direct_messages SET is_read = TRUE WHERE sender_id = $1 AND recipient_id = $2",
    )
    .bind(sender_id)
    .bind(recipient_id)
    .execute(pool)
    .await?;
    Ok(())
}
