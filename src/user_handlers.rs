use std::sync::{Arc, Mutex};



use argon2::Argon2;
use argon2::password_hash::SaltString;
use argon2::password_hash::PasswordHasher;


use bcrypt::DEFAULT_COST;
use bcrypt::hash;
use rand::Rng;


use axum::Extension;
use axum::{Json, response::IntoResponse};
use rand::thread_rng;
use sqlx::{PgPool, Pool};
use sqlx::postgres::Postgres;
use serde_json::json;
use sqlx::{query, query_as};

use axum::extract::{Path, State};

use diesel::prelude::*;

use crate::model::{ NewUser, User, EditUser, UserEmail};






pub async fn get_users(pool: Arc<PgPool>) -> impl IntoResponse {
    let users: Vec<User> = query_as!(
        User,
        r#"
        SELECT id, first_name, last_name, email, password, registration_date FROM users
        "#
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch users");

    Json(users)
}


pub async fn create_user(Json(new_user): Json<NewUser>, pool: Arc<PgPool>) -> impl IntoResponse {
    let first_name = new_user.first_name;
    let last_name = new_user.last_name;
    let email = new_user.email;
    let password = new_user.password;
    let registration_date = new_user.registration_date;


    let users_emails: Option<UserEmail> = query_as!(
        UserEmail,
        "SELECT email FROM users WHERE email = $1",
        email
    )
    .fetch_optional(&*pool)
    .await
    .expect("Failed to fetch user");





    if users_emails.is_some() {
        // Email already exists
        Json(json!({
            "status": "error",
            "message": "User with this email already exists"
        }))
    }else{

        let hashed_password = hash_password(&password);

        let query_result = query!(
       
            "
            INSERT INTO users (first_name, last_name, email, password, registration_date)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
            first_name,
            last_name,
            email,
            hashed_password,
            registration_date
            
        )
        .fetch_one(&*pool)
        .await;
    
        match query_result {
            Ok(row) => {
                let new_id = row.id;
                Json(json!({
                    "status": "success",
                    "message": "Todo created successfully",
                    "new_id": new_id
                }))
            }
            Err(_) => {
                // Handle error case
                // You can return an error response or customize it as needed
                // For now, let's return a generic error response
                Json(json!({
                    "status": "error",
                    "message": "Failed to create todo"
                }))
            }
        }

    }

    
}


pub async fn edit_user(
    Path(id): Path<i32>,
    edit_user_data: Json<EditUser>,
    pool: Arc<PgPool>
) -> impl IntoResponse {
    
    let first_name = edit_user_data.first_name.clone();
    let last_name = edit_user_data.last_name.clone();
    let email = edit_user_data.email.clone();
    // let completed = edit_todo_data.completed;

    let update_result = query!(
        "
        UPDATE users
        SET first_name = coalesce($2, first_name),
            last_name = coalesce($3, last_name),
            email = coalesce($4, email)
        WHERE id = $1
        RETURNING *",
        id,
    first_name,
        last_name,
        email
    )
    .fetch_one(&*pool)
    .await;

    if update_result.is_ok() {
        // Return a success response
        Json(json!({
            "status": "success",
            "message": format!("User with ID {} edited", id)
        }))
    } else {
        // Return an error response
        Json(json!({
            "status": "error",
            "message": "Failed to update user"
        }))
    }


}

// // #[axum_macros::debug_handler] 
pub async fn delete_user(
    Path(id): Path<i32>, 
    pool: Arc<PgPool>
// Extension(db_pool): Extension<Arc<Mutex<PgPool>>>
) -> impl IntoResponse {
    println!("delete_todo id = {}", id);


    
    
    // Use the id to delete the item from the database
    let delete_result = query!(
        "DELETE FROM users WHERE id = $1",
        id
    )
    .execute(&*pool)
    .await;

    if delete_result.is_ok() {
        // Return a success response
        Json(json!({
            "status": "success",
            "message": format!("User with ID {} deleted", id)
        }))
    } else {
        // Return an error response
        Json(json!({
            "status": "error",
            "message": format!("Failed to delete user with ID {}", id)
        }))
    }
   
}


fn hash_password(password: &str) -> String {
    // Generate a new random salt
    // let salt = bcrypt::gen_salt(DEFAULT_COST).expect("Failed to generate salt");

    // Hash the password using bcrypt
    // let password_hash = hash(password, &salt).expect("Failed to hash password");
    let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");
    password_hash
}


// fn hash_password(password: &str) -> String {
//     // Create a new Argon2 hasher
//     let argon2 = Argon2::default();

//     // Generate a new random salt
//     let salt = generate_salt();

//     // Hash the password using Argon2
//     let password_hash = argon2
//         .hash_password(password.as_bytes(), &salt)
//         .expect("Failed to hash password");

//     password_hash.to_string();

// password_hash.to_string()
// }

// fn generate_salt() -> SaltString {
//     SaltString::generate(&mut rand::thread_rng())
// }
