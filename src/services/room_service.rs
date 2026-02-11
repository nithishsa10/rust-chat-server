use uuid::Uuid;

use crate::db::DbPool;
use crate::models::room::{Room, CreateRoomRequest, RoomMember, UpdateRoomRequest};
use crate::error::{Result, AppError};
use crate::repositories::room_repo;


pub async fn create_room(pool: &DbPool, req: &CreateRoomRequest, user_id: Uuid) -> Result<Room> {
    if req.name.is_empty() || req.name.len() > 100 {
        return Err(AppError::BadRequest("Room name must be 1-100 characters".into()));
    }
    let room = room_repo::create_room(
        &pool.pg,
        &req.name,
        req.description.as_deref(),
        req.is_private.unwrap_or(false),
        user_id,
    )
    .await?;
    room_repo::add_room_member(&pool.pg, room.id, user_id, "admin").await?;
    Ok(room)
}

pub async fn get_room(pool: &DbPool, room_id: Uuid) -> Result<Room> {
    room_repo::get_room(&pool.pg, room_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Room not found".into()))
}