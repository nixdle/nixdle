use axum::{
  Json, Router,
  extract::State,
  response::IntoResponse,
  routing::{get, post},
};
use std::{env, fs};

use nixdle::{State as GameState, parse_builtin_types, parse_functions_filtered};

const HOSTNAME: &str = "0.0.0.0:8000";
const HOST: &str = "http://0.0.0.0:8000";

#[derive(serde::Deserialize)]
struct AttemptData {
  input: String,
  attempts: u8,
}

#[tokio::main]
async fn main() {
  println!(
    "WARNING!! this server is for development/example purposes only and should not be used in production"
  );
  let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "lib/data".to_string());
  #[cfg(debug_assertions)]
  println!("using {} as data dir", data_dir);

  let functions =
    parse_functions_filtered(&fs::read_to_string(format!("{}/functions.json", data_dir)).unwrap())
      .unwrap();
  let builtin_types =
    parse_builtin_types(&fs::read_to_string(format!("{}/builtin_types.json", data_dir)).unwrap())
      .unwrap();

  let mut game_state = GameState::new(functions, builtin_types);
  game_state.init_random_game();

  println!("initialized new game");
  println!("{}", game_state.game.clone().unwrap().get_func());

  let app = Router::new()
    .route("/", get(|| async { "hai :3" }))
    .route("/start", get(start_handler))
    .route("/attempt", post(attempt_handler))
    .with_state(game_state);

  println!("listening on http://{}", HOSTNAME);
  let listener = tokio::net::TcpListener::bind(HOSTNAME).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn start_handler(State(state): State<GameState>) -> impl IntoResponse {
  Json(state.start_game(format!("{}/attempt", HOST)))
}

async fn attempt_handler(
  State(state): State<GameState>,
  Json(data): Json<AttemptData>,
) -> impl IntoResponse {
  let response = state.attempt_game(&data.input, data.attempts);
  Json(response.unwrap())
}
