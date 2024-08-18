use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String
}

#[derive(Serialize)]
pub struct CreateChatResponse {
    pub chat_id: i32
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    pub message_id: i32
}

#[derive(Serialize)]
pub struct SingleRetrievedChat {
    pub chat_id: i32,
    pub friend_username: String,
    pub last_message: String,
    pub last_message_time: NaiveDateTime,
}
