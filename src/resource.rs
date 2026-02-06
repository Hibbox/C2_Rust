
use axum::{
    Json, extract::{Path, State}, http::StatusCode
};

use serde::{ Deserialize, Serialize };
use sqlx::{ FromRow, PgPool };
use sqlx::types::{Uuid, chrono::NaiveDateTime};



// Structure pour la base de donner
#[derive(Debug, Deserialize,Serialize, FromRow)]
pub struct Task{
      pub task_id: Uuid,
      pub task_type: String,
      pub task_options: serde_json::Value
}
//Structure pour les result (Post /result)
#[derive(Debug,Serialize,Deserialize, FromRow)]
pub struct ResultTask{
    pub result_id: Uuid,
    pub task_id: Uuid,
    pub sucess: bool,
    pub output: serde_json::Value,
    pub completed_at: NaiveDateTime,
    pub execution: Option<i32>
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
    pub created_at: NaiveDateTime,
    pub sucess: bool,
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
            // Si pas d'options, mettre un objet JSON vide
    
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


pub async fn get_result(State(pool): State<PgPool>) -> Result<Json<Vec<ResultTask>>, StatusCode>{
    let results2 = sqlx::query_as::<_,ResultTask>("SELECT result_id, task_id, sucess, output, completed_at, execution FROM task_result ORDER BY completed_at DESC LIMIT 10")
    .fetch_all(&pool).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(results2))

}


pub async fn add_result(State(pool): State<PgPool>, Json(results): Json<Vec<ResultTask>>
) -> Result<Json<Vec<Task>>, StatusCode>
{

    //recuperer les taches en attentes dans un premier temp avant d'executer les taches
    if results.is_empty(){
        //on doit recuperer les taches en attentes
        let pending_tasks = sqlx::query_as::<_,Task>("SELECT * FROM task ORDER BY task_id ASC LIMIT 10")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(pending_tasks));
    }
    // pour chaque traitement nous devons y attribuer un uuid et y inserer le resultat dans la table task
        for result_payload in results{
            let result_uuid = Uuid::new_v4();
            let _result = sqlx::query("INSERT INTO task_result (result_id, task_id, sucess, output, execution) VALUES ($1, $2, $3, $4, $5)")
            .bind(result_uuid)
            .bind(result_payload.task_id)
            .bind(&result_payload.sucess)
            .bind(&result_payload.output)
            .bind(result_payload.execution)
            .execute(&pool).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            //supprimer la tache de la table active
            sqlx::query("DELETE FROM task WHERE task_id = $1")
            .bind(result_payload.task_id)
            .execute(&pool).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    
        let waiting_tasks = sqlx::query_as::<_,Task>("SELECT * FROM task ORDER BY task_id ASC LIMIT 10")
            .fetch_all(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(waiting_tasks))
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
