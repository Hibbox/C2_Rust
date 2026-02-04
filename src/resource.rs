
use axum::{
    Json, body, extract::{Path, State}, http::StatusCode
};

use serde::{ Deserialize, Serialize };
use sqlx::{ FromRow, PgPool };
use sqlx::types::Uuid;
use chrono;
use uuid;


// Structure pour la base de donner
#[derive(Debug, Deserialize,Serialize, FromRow)]
pub struct Task{
      pub task_id: Uuid,
      pub task_type: String,
      pub task_options: serde_json::Value
}
//Structure pour les result (Post /result)
#[derive(Deserialize, Debug)]
pub struct ResultPayload{
    pub task_id: i32,
    pub status: String,
    pub output: serde_json::Value, 
}
// Structure pour request entrant ( Post /Task)
#[derive(Deserialize, Debug)]
pub struct TaskPayload{
    pub task_type: String,
    pub task_options: serde_json::Value , 
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TaskHistory {
    pub task_id: Uuid,
    pub task_type: String,
    pub task_options: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Deserialize)] //servira pour creer utilisateur
pub struct UserPayload {
    pub name: String,
    pub email: String,
}


#[derive(Serialize, FromRow)] // servira pour obtenir une liste d'utilisateur
pub struct User{
    pub id: i32,
    pub name: String,
    pub email: String
}


pub async fn get_task(State(pool): State<PgPool>,) -> Result<Json<Vec<Task>>, StatusCode> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM task ORDER BY task_id DESC LIMIT $1")
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(tasks))
}

pub async fn add_task(State(pool): State<PgPool>, Json(payloads): Json<Vec<TaskPayload>>) -> Result<Json<Vec<Task>>, StatusCode> 
{
    
    if payloads.is_empty(){
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut added_tasks = Vec::new();

    for p in payloads {
        let task_uuid = Uuid::new_v4();
        
        // Insérer dans la BD
        let result = sqlx::query(
            "INSERT INTO task (task_id, task_type, task_options) VALUES ($1, $2, $3)"
        )
        .bind(task_uuid)
        .bind(&p.task_type)
        .bind(&p.task_options)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if result.rows_affected() > 0 {
            // Créer l'objet Task pour la réponse
            added_tasks.push(Task {
                task_id: task_uuid,
                task_type: p.task_type.clone(),
                task_options: p.task_options.clone(),
            });
        }
    }

    // Retourner les tasks ajoutées
    Ok(Json(added_tasks))
}




pub async fn add_result(State(pool): State<PgPool>, Json(tasks): Json<Vec<serde_json::Value>>
) -> Result<Json<Vec<Task>>, StatusCode>{
    //implement logique
    }

pub async fn get_result(State(pool): State<PgPool>,) -> Result<Json<Vec<Task>>, StatusCode>{
        //implement logique
}


pub async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
        sqlx::query_as::<_, User>(
            "Select * FROM users"
        )
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

pub async fn get_users_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>(
        "Select * FROM users Where id = $1"
    )
    .bind(id)
    .fetch_one(&pool).await
    .map(Json)
    .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn create_user(State(pool): State<PgPool>, Json(payload):Json<UserPayload>) -> Result<(StatusCode, Json<User>), StatusCode>
{
    sqlx::query_as::<_,User>("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
    .bind(payload.name)
    .bind(payload.email)
    .fetch_one(&pool).await
    .map(|u| (StatusCode::CREATED,Json(u)))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_user(State(pool): State<PgPool>, Path(id): Path<i32>, Json(payload):Json<UserPayload>) -> Result<Json<User>, StatusCode>{
    sqlx::query_as::<_,User>("UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *")
    .bind(payload.name)
    .bind(payload.email)
    .bind(id)
    .fetch_one(&pool).await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode>
{
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
    .bind(id)
    .execute(&pool).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0{
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
