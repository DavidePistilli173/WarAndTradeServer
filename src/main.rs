pub mod game;
pub mod game_manager;
pub mod game_state;
pub mod protocol;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use crossbeam_channel::{Receiver, Sender, unbounded};
use game_manager::GameManager;
use game_state::GameState;
use rwlog::Level;
use rwlog::sender::Logger;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::net::TcpListener;

#[derive(Clone)]
struct WebSocketState {
    game_state: Arc<Mutex<GameState>>,
    cmd_tx: Sender<protocol::cmd::Cmd>,
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<WebSocketState>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| async move {
        handle_socket(socket, &state).await;
    })
}

async fn handle_socket(mut socket: WebSocket, game_state: &WebSocketState) {
    // Send a greeting message to the client
    if let Err(e) = socket.send(Message::text("Hello from the server!")).await {
        eprintln!("Error sending message: {}", e);
        return;
    }

    // Loop to keep the connection alive
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(msg) => {
                println!("Received message: {}", msg);
                if let Err(e) = socket.send(Message::text(format!("Echo: {}", msg))).await {
                    eprintln!("Error sending message: {}", e);
                }
            }
            Message::Close(_) => {
                println!("Closing WebSocket connection.");
                break;
            }
            _ => {}
        }
    }
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[derive(Serialize, Deserialize)]
struct MyStruct {
    my_field: String,
}

fn run_game_thread(
    logger: Logger,
    shared_state: Arc<Mutex<GameState>>,
    cmd_rx: Receiver<protocol::cmd::Cmd>,
) {
    thread::spawn(move || {
        let mut game_manager = GameManager::new(logger, shared_state, cmd_rx);
        game_manager.run();
    });
}

async fn run_http_server(
    logger: Logger,
    shared_state: Arc<Mutex<GameState>>,
    cmd_tx: Sender<protocol::cmd::Cmd>,
) {
    let websocket_state = WebSocketState {
        game_state: shared_state.clone(),
        cmd_tx: cmd_tx.clone(),
    };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/ws", get(websocket_handler))
        .with_state(websocket_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let tcp = match TcpListener::bind(&addr).await {
        Ok(x) => x,
        Err(err) => {
            rwlog::fatal!(&logger, "Failed to bind the HTTP server: {err}");
            return;
        }
    };

    if let Err(err) = axum::serve(tcp, router).await {
        rwlog::fatal!(&logger, "Failed to start the HTTP server: {err}");
    }
}

#[tokio::main]
async fn main() {
    let logger = Logger::to_console(Level::Trace);
    rwlog::info!(&logger, "Welcome to the WarAndTrade server!");

    let shared_state = Arc::new(Mutex::new(GameState::new()));
    let (cmd_tx, cmd_rx) = unbounded();

    // Create the game logic thread.
    run_game_thread(logger.clone(), shared_state.clone(), cmd_rx);

    // Create the HTTP server.
    run_http_server(logger.clone(), shared_state, cmd_tx).await;
}
