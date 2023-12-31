use super::game::{get_games, get_game_by_id, create_game, delete_game};

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api/games")
        .service(get_games)
        .service(get_games_by_id)
        .service(create_game)
        .service(update_game)
        .service(delete_game);

    conf.service(scope);
}