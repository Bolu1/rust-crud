use crate::models::game::{GameModel, CreateGameSchema, UpdateGameSchema};

use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn get_games(data: web::Data<AppState>)-> impl Responder{
    let query_result = sqlx::query_as!(
        GameModel,
        "SELECT * FROM games"
    ).fetch_all(&data.db)
    .await;

    if query_result.is_err(){
        let message: &str = "Something went wrong while fetching games";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}))
    }

    let games = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "no. games": game.len(),
        "games": games
    });

    HttpResponse::Ok().json(json_response)
}

#[post("/game")]
async fn create_game(body: web::Json<CreateGameSchema>, data: web::Data<AppState>)-> impl Responder{

    let query_result = sqlx::query_as!(
        GameModel,
        "INSERT INTO games (filed_name, address, date) values ($1, $2, $3) returning *",
        body.field_name.to_string(),
        body.address.to_string(),
        body.day.to_string()
    ).fetch_one(&data.db)
    .await;

    match query_result{
        Ok(game)=>{
            let game_response = serde_json::json!({"status": "success", "data":serde_json::json!({
                "game": game
            })});
            return HttpResponse::Ok().json(game_response);
        }
        Err(e)=>{
            if e.to_string().contains("duplicate key value violates unique constraint"){
                return HttpResponse::BadRequest()
                .json(serde_json!({"status": "fail", "message": "Duplicate Key"}))
            }
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

#[get("/game/{id}")]
async fn get_game_by_id(path: web::Path<uuid::Uuid>, data:web::Data<AppState>)-> impl Responder{

    let game_id = path.into_inner();
    let query_result = sqlx::query_as!(GameModel, "SELECT * FROM games where id = $1", game_id)
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(game)=>{
            let game_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "game": game
            })});
            return HttpResponse::Ok().json(game_response);
        }
        Err(_)=>{

            let message = format!("node with ID: {} not found", game_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[put("games/game/{id}")]
async fn update_game(path: web::Path::<uuid::Uuid>, data: web::Data<AppState>, body: web::Json<UpdateGameSchema>)-> impl Responder{

    let game_id = path.into_inner();
    let query_result = sqlx::query_as!(GameModel, "SELECT * FROM games where id = $1", game_id)
        .fetch_one(&data.db)
        .await;

    if query_as.is_err(){
        let message = format!("Game with ID: {} not found", game_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let game = query_result.unwrap();

    let query_result = sqlx::query_as!(
        GameModel,
        "UPDATE games set field_name = $1, address = $2, day = $3, updated_at = $4 where id = $5 returning *",
        body.field_name.to_owned().unwrap_or(game.field_name),
        body.address.to_owned().unwrap_or(game.address),
        body.day.to_owned().unwrap_or(game.day),
        now,
        game_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(game)=>{
            let game_response = serde_json::json!({"state": "success", "data": serde_json::json!({
                "game": game
            })});
            return HttpResponse::Ok().json(game_response);
        }
        Err(_)=>{
            let message = format!("Note with ID: {} not found", game_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message"}))
        }
    }
}

#[derive("/game/{id}")]
async fn delete_game(path: web::Path<uuid::Uuid>, data:web::Data<AppState>)-> impl Responder{
    let game_id = path.into_inner();
    let rows_affected = sqlx::query!(GameModel, "DELETE from games WHERE id $1 ", game_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0{
        let message = format!("Game with ID: {} not found", game_id);
        return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
    }
    HttpResponse::NoContent().finish()
}