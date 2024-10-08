use sea_orm::DatabaseConnection;
use actix_web::{web, HttpResponse, Error};
use crate::types::globals::AuthenticatedUser;
use crate::utils::auth_helpers::auth_cache::AuthCache;
use crate::utils::auth_helpers::{hash::Hash, jwt::sign_jwt};
use crate::types::{incoming_requests::LoginRequest, outgoing_responses::AuthResponse};
use crate::db::{read::credentials::find_user_by_contact, write::login_history::add_login_history};

pub async fn login(
    db: web::Data<DatabaseConnection>,
    req: web::Json<LoginRequest>,
    auth_cache: web::Data<AuthCache>
) -> Result<HttpResponse, Error> {
    let user = find_user_by_contact(&db, &req.contact).await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "message": "Invalid credentials"
            })));
        }
    };

    let do_passwords_match = Hash::check_password(&req.password, &user.password)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if !do_passwords_match {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Wrong password"
        })));
    }

    let access_token = sign_jwt(&user.user_id)?;

    add_login_history(&db, user.user_id).await?;

    let response = AuthResponse {
        access_token,
        username: user.username.clone()
    };

    auth_cache.store_user(AuthenticatedUser(user)).await;

    Ok(HttpResponse::Ok().json(response))
}
