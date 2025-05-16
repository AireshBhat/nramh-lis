use crate::handler::meril_handler::MerilHandler;
use tokio::sync::Mutex;

/// Application state shared between Tauri commands
pub struct AppState {
    /// MerilHandler for managing Meril machine communications
    pub meril_handler: Mutex<MerilHandler>,
}

impl AppState {
    /// Create a new AppState with the given MerilHandler
    pub fn new(meril_handler: MerilHandler) -> Self {
        Self {
            meril_handler: Mutex::new(meril_handler),
        }
    }
}
