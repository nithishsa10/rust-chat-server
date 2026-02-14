use sqlx::PgPool;
use uuid::Uuid;
use crate::models::room::{Room, RoomMember};
use crate::error::Result;

pub async fn create_room(
    pool: &PgPool,
    room_name: &str,
    description: Option<&str>,
    is_private: bool,
    create_by: Uuid
) -> Result<Room> {
    let room = sqlx::query_as::<_, Room>(
        r#"
            INSERT INTO rooms (name, description, is_private, create_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
    )
    .bind(room_name)
    .bind(description)
    .bind(is_private)
    .bind(create_by)
    .fetch_one(pool)
    .await?;
    Ok(room)
}

pub async fn get_room(pool: &PgPool, id: Uuid) -> Result<Option<Room>> {
    Ok(sqlx::query_as::<_, Room>("SELECT * FROM rooms WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?)
}

pub async fn list_room(pool: &PgPool, include_private: bool, limit: i32) -> Result<Vec<Room>> {
    if include_private {
        Ok(sqlx::query_as::<_, Room>("SeLECT * FROM rooms ORDER BY created_at DESC LIMIT $1")
        .fetch_all(pool)
        .await?
        )
    } else {
        Ok(sqlx::query_as::<_, Room>("SeLECT * FROM rooms WHERE is_private = false ORDER BY created_at DESC LIMIT $1")
        .bind(limit)
        .fetch_all(pool)
        .await?
        )
    }
}

pub async fn update_room(
    pool: &PgPool,
    id: Uuid,
    name: Option<&str>,
    description: Option<&str>,
    is_private: Option<bool>
) -> Result<Room> {
    let room = sqlx::query_as::<_, Room>(
        r#"
        UPDATE rooms
        SET name        = COALESCE($2, name),
            description = COALESCE($3, description)
            is_private  = COALESCE($4, is_private)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(is_private)
    .fetch_one(pool)
    .await?;
    Ok(room)
}

pub async fn delete_room(pool: &PgPool, id: Uuid) -> Result<u64> {
    let result = sqlx::query("DELETE FROM rooms WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub async fn add_room_member(pool: &PgPool, room_id: Uuid, user_id: Uuid, role: &str) -> Result<()> {
    sqlx::query(
        "INSERT INTO room_members (room_id, user_id, role) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
    )
    .bind(room_id)
    .bind(user_id)
    .bind(role)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_room_member(pool: &PgPool, room_id: Uuid, user_id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM room_members WHERE room_id = $1 AND user_id = $2")
        .bind(room_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_room_members(pool: &PgPool, room_id: Uuid) -> Result<Vec<RoomMember>> {
    Ok(sqlx::query_as::<_, RoomMember>("SELECT * FROM room_members WHERE room_id = $1")
        .bind(room_id)
        .fetch_all(pool)
        .await?)
}

pub async fn is_room_member(pool: &PgPool, room_id: Uuid, user_id: Uuid) -> Result<bool> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM room_members WHERE room_id = $1 AND user_id = $2",
    )
    .bind(room_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(count > 0)
}


pub async fn get_member_role(pool: &PgPool, room_id: Uuid, user_id: Uuid) -> Result<Option<String>> {
    Ok(sqlx::query_scalar::<_, String>(
        "SELECT role FROM room_members WHERE room_id = $1 AND user_id = $2",
    )
    .bind(room_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?)
}
