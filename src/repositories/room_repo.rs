use sqlx::PgPool;
use uuid::Uuid;
use crate::models::room::Room;
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
        "#r
        Update rooms
        SET name = $2, description = $3, is_private = $4
        WHERE id = $1
        RETURNING *
        ",
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(is_private)
    .fetch_one(pool)
    .await?;
    Ok(room)
}

pub async fn delete_room(pool: &PgPool, id: Uuid) -> Result<Room> {
    let room = sqlx::query_as::<_, Room>("DELETE FROM rooms WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(room)
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
