use std::sync::Mutex;

use actix_web::{post,Responder,HttpResponse, web::{Json, self}};
use crate::{models, AppState};
use models::user::{User,LoginData};


#[post("/register")]
pub async fn register_user(data: web::Data<Mutex<AppState>>, user: Json<User>) -> impl Responder {
    let insert_result = data.lock().unwrap().user_repo.create_user(user.into_inner()).await;

    match insert_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string())
    }
}

#[post("/login")]
pub async fn login_user(data: web::Data<Mutex<AppState>>, credentials: Json<LoginData>) -> impl Responder {
    let user = data.lock().unwrap().user_repo.get_user(credentials.into_inner()).await;

    match user {
        Ok(loggedin) => HttpResponse::Ok().json(loggedin),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string())
    }
}
