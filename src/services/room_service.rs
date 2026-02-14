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

pub async fn list_room(pool: &DbPool, limit: Option<i32>) -> Result<Vec<Room>> {
    room_repo::list_room(&pool.pg, false, limit.unwrap_or(10))
        .await
        .map_err(|e| AppError::from(e))
}

pub async fn update_room(pool: &DbPool, room_id: Uuid, user_id: Uuid, req: &UpdateRoomRequest) -> Result<Room> {
    ensure_user_admin_or_creator(pool, room_id, user_id).await?;
    room_repo::update_room(&pool.pg, room_id, req.name.as_deref(), req.description.as_deref(), req.is_private)
        .await
        .map_err(|e| AppError::from(e))
}

pub async fn delete_room(pool: &DbPool, room_id: Uuid, user_id: Uuid) -> Result<()> {
    ensure_user_admin_or_creator(pool, room_id, user_id).await?;
    let affected = room_repo::delete_room(&pool.pg, room_id)
        .await?;
    if affected == 0 {
        return Err(AppError::NotFound("Room not found".into()));
    }
    Ok(())
}

pub async fn join_room(pool: &DbPool, room_id: Uuid, user_id: Uuid) -> Result<()> {
    let room = room_repo::get_room(&pool.pg, room_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Room not found".into()))?;

    if room.is_private && !room_repo::is_room_member(&pool.pg, room_id, user_id).await? {
        return Err(AppError::Forbidden("Room is private".into()));
    }
    room_repo::add_room_member(&pool.pg, room_id, user_id, "member").await
}

pub async fn leave_room(pool: &DbPool, room_id: Uuid, user_id: Uuid) -> Result<()> {
    room_repo::remove_room_member(&pool.pg, room_id, user_id).await
}

pub async fn get_room_members(pool: &DbPool, room_id: Uuid) -> Result<Vec<RoomMember>> {
    room_repo::get_room(&pool.pg, room_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Room not found".into()))?;
    room_repo::get_room_members(&pool.pg, room_id).await
}


async fn ensure_user_admin_or_creator(pool: &DbPool, room_id: Uuid, user_id: Uuid) -> Result<()> {
    let room = room_repo::get_room(&pool.pg, room_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Room not found".into()))?;
    if room.created_by == user_id {
        return Ok(())
    }
    match room_repo::get_member_role(&pool.pg, room_id, user_id).await? {
        Some(ref role) if role == "admin" => Ok(()),
        _ => Err(AppError::Forbidden("Only room admin can perform this action".into())),
    }
}