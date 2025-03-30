pub mod game;
pub mod game_manager;
pub mod protocol;
pub mod ui;

use crossbeam_channel::{Receiver, Sender, unbounded};
use game_manager::GameManager;
use protocol::cmd;
use rwlog::Level;
use rwlog::sender::Logger;
use std::thread::{self, JoinHandle};
use ui::watui::WATUI;

fn run_game_thread(
    logger: Logger,
    cmd_rx: Receiver<protocol::cmd::Cmd>,
    tlm_tx: Sender<protocol::tlm::Tlm>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut game_manager = GameManager::new(logger, cmd_rx, tlm_tx);
        game_manager.run();
    })
}

fn main() {
    let logger = Logger::to_console(Level::Trace);
    rwlog::info!(&logger, "Welcome to the WarAndTrade!");

    let (cmd_tx, cmd_rx) = unbounded(); // Channel for sending commands to the server.
    let (tlm_tx, tlm_rx) = unbounded(); // Channel for receiving updates from the server.

    // Create the game logic thread.
    let server_thread = run_game_thread(logger.clone(), cmd_rx, tlm_tx);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("WarAndTrade")
            .with_inner_size((1600f32, 900f32)),
        ..Default::default()
    };
    if let Err(err) = eframe::run_native(
        "WarAndTrade",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(WATUI::new(
                cc,
                logger.clone(),
                cmd_tx.clone(),
                tlm_rx.clone(),
            )))
        }),
    ) {
        rwlog::fatal!(
            &logger,
            "Error while running the application window: {err}."
        );
    }

    if let Err(err) = cmd_tx.send(cmd::Cmd::CloseServer()) {
        rwlog::err!(&logger, "Failed to stop server: {err}.");
    }
    let _ = server_thread.join();
}
