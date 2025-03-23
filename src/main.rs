pub mod game;
pub mod game_manager;
pub mod game_state;
pub mod protocol;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::{Router, extract::State, routing::get};
use crossbeam_channel::{Receiver, Sender, TryRecvError, unbounded};
use game_manager::GameManager;
use rwlog::Level;
use rwlog::sender::Logger;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::thread;
use tokio::net::TcpListener;

#[derive(Clone)]
struct CmdWebsocketState {
    logger: Logger,
    cmd_tx: Sender<protocol::cmd::Cmd>,
}

#[derive(Clone)]
struct TlmWebsocketState {
    logger: Logger,
    tlm_rx: Receiver<protocol::tlm::Tlm>,
}

async fn cmd_websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<CmdWebsocketState>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| async move {
        handle_cmd_websocket(socket, &state).await;
    })
}

async fn handle_cmd_websocket(mut socket: WebSocket, state: &CmdWebsocketState) {
    // Wait for commands.
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(msg) => match serde_json::from_str(&msg) {
                Ok(cmd) => {
                    if let Err(e) = state.cmd_tx.send(cmd) {
                        rwlog::err!(&state.logger, "Failed to send command: {e}");
                    }
                }
                Err(e) => {
                    rwlog::err!(&state.logger, "Failed to parse command: {e}");
                }
            },
            Message::Close(_) => {
                rwlog::info!(&state.logger, "Closing WebSocket connection.");
                break;
            }
            _ => {}
        }
    }
}

async fn tlm_websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<TlmWebsocketState>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| async move {
        handle_tlm_websocket(socket, state).await;
    })
}

async fn handle_tlm_websocket(mut socket: WebSocket, game_state: TlmWebsocketState) {
    loop {
        let tlm = match game_state.tlm_rx.try_recv() {
            Ok(tlm) => {
                rwlog::trace!(&game_state.logger, "Sending telemetry.");
                tlm
            }
            Err(e) => {
                match e {
                    TryRecvError::Disconnected => {
                        rwlog::info!(&game_state.logger, "Telemetry channel disconnected.");
                        break;
                    }
                    _ => {}
                }
                break;
            }
        };

        if let Err(e) = socket
            .send(Message::text(serde_json::to_string(&tlm).unwrap()))
            .await
        {
            rwlog::err!(&game_state.logger, "Failed to send telemetry: {e}");
            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
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
    cmd_rx: Receiver<protocol::cmd::Cmd>,
    tlm_tx: Sender<protocol::tlm::Tlm>,
) {
    thread::spawn(move || {
        let mut game_manager = GameManager::new(logger, cmd_rx, tlm_tx);
        game_manager.run();
    });
}

async fn run_http_server(
    logger: Logger,
    cmd_tx: Sender<protocol::cmd::Cmd>,
    tlm_rx: Receiver<protocol::tlm::Tlm>,
) {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/cmd", get(cmd_websocket_handler))
        .with_state(CmdWebsocketState {
            logger: logger.clone(),
            cmd_tx,
        })
        .route("/tlm", get(tlm_websocket_handler))
        .with_state(TlmWebsocketState {
            logger: logger.clone(),
            tlm_rx,
        });
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

    let (cmd_tx, cmd_rx) = unbounded();
    let (tlm_tx, tlm_rx) = unbounded();

    // Create the game logic thread.
    run_game_thread(logger.clone(), cmd_rx, tlm_tx);

    // Create the HTTP server.
    run_http_server(logger.clone(), cmd_tx, tlm_rx).await;
}
