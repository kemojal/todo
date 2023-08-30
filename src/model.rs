


use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{ FromRow};

#[derive(Deserialize)]
pub  struct NewUser {
    pub  first_name: String,
    pub  last_name: String,
    pub  email: String,
    pub  password: String,
    pub  registration_date: NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub  struct User {
    pub  id: i32,
    pub  first_name: Option<String>,
    pub  last_name: Option<String>,
    pub  email: Option<String>,
    pub  password: Option<String>,
    pub registration_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, FromRow)]
pub  struct EditUser {
    pub  first_name: Option<String>,
    pub  last_name: Option<String>,
    pub  email: Option<String>,
    // pub  password: Option<String>,
    // pub registration_date: Option<NaiveDateTime>,
}


#[derive(Deserialize)]
pub  struct UserEmail {
    pub  email: Option<String>,
}




#[derive(Deserialize)]
pub  struct NewTodo {
    pub  title: String,
    pub  description: String,
    pub  completed: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub  struct TodoItem {
    pub  id: i32,
    pub  title: String,
    pub  description: Option<String>,
    pub  completed: bool,
}

// Inside your model module (model.rs)
#[derive(Debug, Deserialize, FromRow)]
pub struct EditTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}



// auth

#[derive(Debug, Deserialize, FromRow)]
pub struct SignInData {
    pub email:  String,
    pub password:  String,
}

#[derive(Debug, Deserialize)]
pub  struct AuthUser {
    pub  email: Option<String>,
    pub  password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub first_name: String,
    pub last_name: String,
}