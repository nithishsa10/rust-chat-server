use uuid::Uuid;

use crate::db::DbPool;
use crate::repositories::{message_repo, room_repo, user_repo};
use crate::models::message::{Message, DirectMessage, MessageEvent, MessageUser, PaginationParams};
use crate::error::{AppError, Result};

pub async fn send_message(pool: &DbPool, user_id: Uuid, room_id: Uuid, content: &str) -> Result<Message> {
    if content.trim().is_empty() || content.len() > 10_000 {
        return Err(AppError::BadRequest("Message content cannot be empty".into()))
    }
    if !room_repo::is_room_member(&pool.pg, room_id, user_id).await? {
        return Err(AppError::Forbidden("You are not a member of this room".into()));
    }
    message_repo::create_message(&pool.pg, room_id, user_id, content).await

}


pub async fn get_messages(pool: &DbPool, room_id: Uuid, params: &PaginationParams) -> Result<Vec<Message>> {
    let limit  = params.limit.unwrap_or(50).min(200) as i64;
    let offset = params.offset.unwrap_or(0).max(0) as i64;
    message_repo::get_room_messages(&pool.pg, room_id, limit, offset).await
}

pub async fn delete_message(pool: &DbPool, message_id: Uuid, user_id: Uuid) -> Result<()> {
    let msg = message_repo::get_message(&pool.pg, message_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Message not found".into()))?;
    if msg.sender_id != user_id {
        return Err(AppError::Forbidden("You can only delete your own messages".into()));
    }
    message_repo::delete_message(&pool.pg, message_id).await?;
    Ok(())
}

/// Build a rich MessageEvent for broadcasting over WebSocket.
pub async fn build_message_event(pool: &DbPool, msg: &Message) -> Result<MessageEvent> {
    let user = user_repo::get_user_by_id(&pool.pg, msg.sender_id)
        .await?
        .ok_or_else(|| AppError::Internal("Message author not found".into()))?;
    Ok(MessageEvent {
        message_id: msg.id,
        room_id:    msg.room_id,
        user: MessageUser {
            id:           user.id,
            username:     user.username,
            display_name: user.display_name,
        },
        content:   msg.content.clone(),
        timestamp: msg.created_at,
    })
}


pub async fn send_dm(pool: &DbPool, sender_id: Uuid, recipient_id: Uuid, content: &str) -> Result<DirectMessage> {
    if content.is_empty() || content.len() > 10_000 {
        return Err(AppError::BadRequest("Message must be 1-10 000 characters".into()));
    }
    if sender_id == recipient_id {
        return Err(AppError::BadRequest("Cannot send a DM to yourself".into()));
    }
    user_repo::get_user_by_id(&pool.pg, recipient_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Recipient not found".into()))?;
    message_repo::create_direct_message(&pool.pg, sender_id, recipient_id, content).await
}

pub async fn get_dm_history(pool: &DbPool, user_a: Uuid, user_b: Uuid) -> Result<Vec<DirectMessage>> {
    message_repo::get_dm_history(&pool.pg, user_a, user_b, 100).await
}

pub async fn mark_dms_read(pool: &DbPool, sender_id: Uuid, recipient_id: Uuid) -> Result<()> {
    message_repo::mark_dms_read(&pool.pg, sender_id, recipient_id).await
}
