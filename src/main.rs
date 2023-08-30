use std::sync::{Arc, Mutex};

use axum::{
    
     routing::{get, post, put, delete},  Json, Router, middleware::AddExtension, Extension, extract::Path
    
};










mod db;
use db::create_db_pool;


mod handlers;
use handlers::{get_todos, create_todo,delete_todo, root, edit_todo};


mod user_handlers;
use user_handlers::{create_user, get_users, edit_user, delete_user};


mod auth;
use auth::{ sign_in, sign_out};


mod model;
use model::{NewTodo, EditTodo, NewUser, EditUser, SignInData};







#[tokio::main]
async fn main() {
    
    let pool = create_db_pool().await;
    let pool_clone = pool.clone(); 
    // let editPool = pool.clone();


    let shared_pool = Arc::new(pool);

    let pool_for_todos = shared_pool.clone();

    let editPool = shared_pool.clone();


    let userPool = shared_pool.clone();
    let getUserPool = shared_pool.clone();
    let editUserPool = shared_pool.clone();
    let deleteUserPool = shared_pool.clone();


    let authPool = shared_pool.clone();
    let signOutPool = shared_pool.clone();

    let app = Router::new()
    .route("/", get(root))
    .route("/api/users", get(move || get_users(getUserPool.clone())))
    .route("/api/user/create", post(move |Json(new_user): Json<NewUser>| {
        create_user(axum::Json(new_user),userPool.clone())
    }))
    .route("/api/user/edit/:id", put(move |path: Path<i32>, Json(edit_user_data): Json<EditUser>| {
        edit_user(path, Json(edit_user_data), editUserPool)
    }))
    .route("/api/user/delete/:id", delete(move |path: Path<i32>| {
       
        delete_user(path,  deleteUserPool)
    }))
    .route("/api/todos", get(move || get_todos(shared_pool.clone())))
    .route("/api/todo/create", post(move |Json(new_todo): Json<NewTodo>| {
        create_todo(axum::Json(new_todo),pool_clone.clone())
    }))
    .route("/api/todo/delete/:id", delete(move |path: Path<i32>| {
        // let pool_clone = shared_pool.clone().clone();
        delete_todo(path, pool_for_todos)
    }))
    .route("/api/todo/edit/:id", put(move |path: Path<i32>, Json(edit_todo_data): Json<EditTodo>| {
        edit_todo(path, Json(edit_todo_data), editPool)
    }))
    .route("/api/auth/signin", post(move |Json(sign_in_data): Json<SignInData>| {
        sign_in(axum::Json(sign_in_data),authPool)
    }))
    .route("/api/auth/signout", post(move |Json(sign_out_data): Json<SignInData>| {
        sign_out(axum::Json(sign_out_data), signOutPool)
    }))
    ;
    // .route("/api/todo/delete/:id", delete(delete_todo));
    // .layer(Extension( shared_pool));

    


    
    
    
    
    

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}




