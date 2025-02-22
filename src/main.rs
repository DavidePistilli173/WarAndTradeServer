pub mod app_manager;
pub mod errors;
pub mod protocol;

use app_manager::AppManager;
use errors::AppManagerInitErr;
use rwlog::Level;
use rwlog::sender::Logger;
use std::thread::sleep;
use std::time::Duration;

fn wait_initial_connection() {}

fn main() {
    let logger = Logger::to_console(Level::Trace);
    rwlog::info!(&logger, "Welcome to the WarAndTrade server!");

    match AppManager::new(logger.clone()) {
        Err(AppManagerInitErr::SocketCreation) => {
            rwlog::fatal!(&logger, "Failed to create the GUI socket.");
        }
        Ok(mut manager) => {
            rwlog::info!(&logger, "Starting the server.");
            manager.run();
        }
    };

    loop {
        sleep(Duration::from_millis(1000));
    }
}
