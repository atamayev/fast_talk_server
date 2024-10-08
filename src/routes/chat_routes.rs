use actix_web::web;
use sea_orm::DatabaseConnection;
use crate::middleware::jwt_verify::JwtVerify;
use crate::middleware::chat_middleware::new_message_middleware::ValidateNewMessage;

use crate::handlers::chat_handlers::create_chat::create_chat;
use crate::handlers::chat_handlers::send_message::send_message;
use crate::handlers::chat_handlers::retrieve_chats_list::retrieve_chats_list;
use crate::handlers::chat_handlers::search_for_usernames::search_for_usernames;
use crate::handlers::chat_handlers::retrieve_chat_messages::retrieve_chat_messages;

pub fn chat_routes(cfg: &mut web::ServiceConfig, db: web::Data<DatabaseConnection>) {
    // Create the JwtVerify middleware instance once
    let jwt_verify = JwtVerify::new(db.clone());

    cfg.service(
        web::scope("/chat")
            .wrap(jwt_verify.clone()) // Apply the middleware to the entire scope
            .service(
                web::resource("/create-chat/{friendId}")
                    .route(web::post().to(create_chat))
            )
            .service(
                web::resource("/send-message/{chatId}")
                    .wrap(ValidateNewMessage)
                    .route(web::post().to(send_message))
            )
            .service(
                web::resource("/retrieve-chats-list")
                    .route(web::get().to(retrieve_chats_list))
            )
            .service(
                web::resource("/retrieve-chat-messages/{chatId}")
                    .route(web::get().to(retrieve_chat_messages))
            )
            .service(
                web::resource("/search-for-usernames/{username}")
                    .route(web::get().to(search_for_usernames))
            )
    );
}
