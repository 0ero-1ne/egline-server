use rocket::serde::json::{Json, serde_json};
use serde_json::json;

use rocket::response::status;
use rocket::http::Status;

use rocket::State;

use crate::entities::{user::Entity, user::Model, user::ActiveModel};

use sea_orm::{prelude::DbErr, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

use sha256::digest;


#[utoipa::path(
    context_path = "/user",
    responses(
        (status = 200, description = "All users", body = Vec<Model>),
        (status = 500, description = "No users", body = String)
    ),
)]
#[get("/")]
async fn get_all_users(
    db: &State<DatabaseConnection>
) -> Result<Json<Vec<Model>>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let users = Entity::find().all(db).await;

    match users {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/<id>")]
async fn get_user_by_id(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<Model>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let user = Entity::find_by_id(id).one(db).await;

    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => {
            let empty_user = Model {
                id: -1,
                display_name: String::new(),
                email: String::new(),
                password: String::new(),
                avatar: Vec::new(),
                saved_books: json!(""),
            };
            Ok(Json(empty_user))
        }
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/", data="<user_data>", format="json")]
async fn create_user(
    db: &State<DatabaseConnection>,
    user_data: Json<Model>,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let hashed_password: String = digest(user_data.password.clone());

    let user:Result<Model, DbErr> = ActiveModel {
        display_name: ActiveValue::set(user_data.display_name.clone()),
        email: ActiveValue::set(user_data.email.clone().to_lowercase()),
        password: ActiveValue::set(hashed_password),
        avatar: ActiveValue::set(user_data.avatar.clone()),
        saved_books: ActiveValue::set(json!(user_data.saved_books.clone())),
        ..Default::default()
    }.insert(db).await;

    match user {
        Ok(_) => Ok(Json(format!("User {} was successfully created", user_data.display_name.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/<id>", data="<user_data>", format="json")]
async fn update_user(
    db: &State<DatabaseConnection>,
    user_data: Json<Model>,
    id: i32,
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;
    let user = Entity::find_by_id(id).one(db).await.unwrap().unwrap();

    let hashed_password = if user_data.password.is_empty() {
        user.password
    } else {
        digest(user_data.password.clone())
    };

    let updated_user = ActiveModel {
        id: ActiveValue::set(id),
        display_name: ActiveValue::set(user_data.display_name.clone()),
        email: ActiveValue::set(user_data.email.clone().to_lowercase()),
        password: ActiveValue::set(hashed_password),
        avatar: ActiveValue::set(user_data.avatar.clone()),
        saved_books: ActiveValue::set(json!(user_data.saved_books.clone())),
        ..Default::default()
    }.update(db).await;

    match updated_user {
        Ok(result) => Ok(Json(format!("User {} was successfully updated", result.display_name.clone()))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[delete("/<id>")]
async fn delete_user(
    db: &State<DatabaseConnection>,
    id: i32
) -> Result<Json<String>, status::Custom<String>> {
    let db: &DatabaseConnection = db as &DatabaseConnection;

    let deleted_user = ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }.delete(db).await;

    match deleted_user {
        Ok(result) => Ok(Json(format!("Number of deleted entries: {}", result.rows_affected))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn get_all_methods() -> Vec<rocket::Route> {
    routes![get_all_users, get_user_by_id, create_user, update_user, delete_user]
}