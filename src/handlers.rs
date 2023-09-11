use std::sync::{Arc};


use axum::{Json, response::IntoResponse};
use sqlx::{PgPool};

use serde_json::json;
use sqlx::{query, query_as};

use axum::extract::{Path};


use crate::model::{TodoItem, NewTodo, EditTodo};





pub async fn get_todos(pool: Arc<PgPool>) -> impl IntoResponse {
    let todos: Vec<TodoItem> = query_as!(
        TodoItem,
        r#"
        SELECT id, title, description, completed, user_id, status,priority, due_date  FROM todo_items
        "#
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch todos");

    Json(todos)
}

pub async fn get_user_todos(
    Path(id): Path<i32>,
    pool: Arc<PgPool>) -> impl IntoResponse {
    let todos =
    query!(
        "SELECT id, title, description, completed, user_id, status,priority, due_date  FROM todo_items  WHERE user_id = $1",
        id
    )
    .fetch_all(&*pool)
    .await;

    match todos {
        Ok(rows) => {
            let todos: Vec<TodoItem> = rows
                .into_iter()
                .map(|row| {
                    let user_id = row.user_id;
                    let id: i32 = row.id;
                    let title: String = row.title;
                    let description: Option<String> = row.description;
                    let completed: bool = row.completed;
                    let status  = row.status;
                    let priority  = row.priority;
                    let due_date = row.due_date;
                    TodoItem {
                        id,
                        title,
                        description,
                        completed,
                        user_id,
                        status,
                        priority,
                        due_date,
                    }
                })
                .collect();

            Json(todos)
        }
        Err(_) => {
            // Handle the error case here, e.g., returning an error response
            // For simplicity, let's return an empty list of todos for now
            Json(Vec::<TodoItem>::new())
        }
    }

    // Json(todos)
}



pub async fn create_todo(Json(new_todo): Json<NewTodo>, pool: PgPool) -> impl IntoResponse {
    let title = new_todo.title;
    let description = new_todo.description;
    let completed = new_todo.completed;
    let user_id = new_todo.user_id;
    let status = new_todo.status;
    let priority = new_todo.priority;
    let due_date = new_todo.due_date;

    let query_result = query!(
       
        "
        INSERT INTO todo_items (title, description, completed, user_id, status, priority, due_date)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *",
        title,
        description,
        completed,
        user_id,
        status,
        priority,
        due_date
        
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
    let user_id = edit_todo_data.user_id;
    let status = edit_todo_data.status.clone();
    let priority = edit_todo_data.priority;
    let due_date = edit_todo_data.due_date;

    let update_result = query!(
        "
        UPDATE todo_items
        SET title = coalesce($2, title),
            description = coalesce($3, description),
            completed = coalesce($4, completed),
            status = coalesce($5, status),
            priority = coalesce($6, priority),
            due_date = coalesce($7, due_date)
        WHERE id = $1 AND user_id = $8
        RETURNING *",
        id,
        title,
        description,
        completed,
        status,
        priority,
        due_date,
        user_id
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