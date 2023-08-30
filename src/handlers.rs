use std::sync::{Arc, Mutex};

use axum::Extension;
use axum::{Json, response::IntoResponse};
use sqlx::{PgPool, Pool};
use sqlx::postgres::Postgres;
use serde_json::json;
use sqlx::{query, query_as};

use axum::extract::{Path, State};


use crate::model::{TodoItem, NewTodo, EditTodo};





pub async fn get_todos(pool: Arc<PgPool>) -> impl IntoResponse {
    let todos: Vec<TodoItem> = query_as!(
        TodoItem,
        r#"
        SELECT id, title, description, completed FROM todo_items
        "#
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch todos");

    Json(todos)
}


pub async fn create_todo(Json(new_todo): Json<NewTodo>, pool: PgPool) -> impl IntoResponse {
    let title = new_todo.title;
    let description = new_todo.description;
    let completed = new_todo.completed;

    let query_result = query!(
       
        "
        INSERT INTO todo_items (title, description, completed)
        VALUES ($1, $2, $3)
        RETURNING *",
        title,
        description,
        completed
        
    )
    .fetch_one(&pool)
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


pub async fn edit_todo(
    Path(id): Path<i32>,
    edit_todo_data: Json<EditTodo>,
    pool: Arc<PgPool>
) -> impl IntoResponse {
    
    let title = edit_todo_data.title.clone();
    let description = edit_todo_data.description.clone();
    let completed = edit_todo_data.completed;

    let update_result = query!(
        "
        UPDATE todo_items
        SET title = coalesce($2, title),
            description = coalesce($3, description),
            completed = coalesce($4, completed)
        WHERE id = $1
        RETURNING *",
        id,
        title,
        description,
        completed
    )
    .fetch_one(&*pool)
    .await;

    if update_result.is_ok() {
        // Return a success response
        Json(json!({
            "status": "success",
            "message": format!("Todo item with ID {} edited", id)
        }))
    } else {
        // Return an error response
        Json(json!({
            "status": "error",
            "message": "Failed to update todo"
        }))
    }

    // match update_result {
    //     Ok(row) => {
    //         // Return the updated todo item
    //         Json(row)
    //     }
    //     Err(_) => {
    //         // Handle error case
    //         // Return an error response
    //         Json(json!({
    //             "status": "error",
    //             "message": "Failed to update todo"
    //         })) // This returns a Json<Value>
    //     }
    // }
}

// #[axum_macros::debug_handler] 
pub async fn delete_todo(
    Path(id): Path<i32>, 
    pool: Arc<PgPool>
// Extension(db_pool): Extension<Arc<Mutex<PgPool>>>
) -> impl IntoResponse {
    println!("delete_todo id = {}", id);


    
    
    // Use the id to delete the item from the database
    let delete_result = query!(
        "DELETE FROM todo_items WHERE id = $1",
        id
    )
    .execute(&*pool)
    .await;

    if delete_result.is_ok() {
        // Return a success response
        Json(json!({
            "status": "success",
            "message": format!("Todo item with ID {} deleted", id)
        }))
    } else {
        // Return an error response
        Json(json!({
            "status": "error",
            "message": format!("Failed to delete todo item with ID {}", id)
        }))
    }
    // Json(json!({
    //     "status": "success",
    //     "message": format!("Todo item with ID {} deleted", id)
    // }))
}

pub async fn root() -> &'static str {
    "Hello, World!"
}