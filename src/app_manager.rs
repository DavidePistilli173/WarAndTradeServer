use crate::errors::AppManagerInitErr;
use crate::game_manager::GameManager;
use crate::protocol::net_interface::NetInterface;
use rwlog::sender::Logger;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Application states.
pub enum AppState {
    /// No game currently running.
    Idle,
    /// Waiting for the game to start.
    GameLoading,
    /// GameManager running.
    GameRunning,
}

/// Main server application manager.
pub struct AppManager {
    /// Logger.
    logger: Logger,
    /// Main network interface for the server.
    net_if: NetInterface,
    /// Actual game object.
    game: GameManager,
}

impl AppManager {
    /// Create a new server manager.
    pub fn new(logger: Logger) -> Result<Self, AppManagerInitErr> {
        let net_if = NetInterface::new(logger.clone()).map_err(|_| {
            rwlog::err!(&logger, "Failed to create the server's network interface.");
            AppManagerInitErr::NetworkInterfaceCreation
        })?;

        let game = GameManager::new(logger.clone());

        Ok(AppManager {
            logger,
            net_if,
            game,
        })
    }

    /// Run the application.
    pub fn run(&mut self) {
        const ITERATION_TIME: Duration = Duration::from_millis(10);

        loop {
            let start = SystemTime::now();
            let start = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            self.game.process_commands(self.net_if.get_commands());

            let end = SystemTime::now();
            let end = end
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards.");

            sleep(ITERATION_TIME - (end - start));
        }
    }
}
