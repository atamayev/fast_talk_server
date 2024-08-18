use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use std::error::Error;
use crate::entities::chats;
use chrono::NaiveDateTime;

pub async fn does_chat_exist(db: &DatabaseConnection, chat_id: i32) -> Result<bool, Box<dyn Error>> {
    let chat = chats::Entity::find()
        .filter(chats::Column::ChatId.eq(chat_id)) 
        .one(db)
        .await?;

    Ok(chat.is_some())
}

#[derive(Debug)]
pub struct ChatInfo {
    pub chat_id: i32,
    pub updated_at: NaiveDateTime,
    pub last_message: Option<String>,
}

pub async fn get_chats_info(
    db: &DatabaseConnection,
    chat_ids: Vec<i32>,
) -> Result<Vec<ChatInfo>, Box<dyn Error>> {
    // Query the chats table for the given chat_ids
    let chats_models = chats::Entity::find()
        .filter(chats::Column::ChatId.is_in(chat_ids))
        .select_only()
        .column(chats::Column::ChatId)
        .column(chats::Column::UpdatedAt)
        .column(chats::Column::LastMessage)
        .into_model::<chats::Model>() // Use the model type provided by the SeaORM entity
        .all(db)
        .await?;

    // Map the results from the model into your custom ChatInfo struct
    let chats_info: Vec<ChatInfo> = chats_models
        .into_iter()
        .map(|chat| ChatInfo {
            chat_id: chat.chat_id,
            updated_at: chat.updated_at.naive_utc(),  // Convert DateTime<FixedOffset> to NaiveDateTime
            last_message: chat.last_message,
        })
        .collect();

    Ok(chats_info)
}
