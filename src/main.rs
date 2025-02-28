pub mod app_manager;
pub mod errors;
pub mod game;
pub mod game_manager;
pub mod protocol;

use app_manager::AppManager;
use errors::AppManagerInitErr;
use rwlog::Level;
use rwlog::sender::Logger;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let logger = Logger::to_console(Level::Trace);
    rwlog::info!(&logger, "Welcome to the WarAndTrade server!");

    // Initialise and run the application manager.
    match AppManager::new(logger.clone()) {
        Err(AppManagerInitErr::NetworkInterfaceCreation) => {
            rwlog::fatal!(
                &logger,
                "Failed to create the main server network interface."
            );
        }
        Ok(mut manager) => {
            rwlog::info!(&logger, "Starting the server.");
            manager.run();
        }
    };

    // Wait in case an error occurs.
    loop {
        sleep(Duration::from_millis(1000));
    }
}
