use std::sync::RwLock;
use std::io::{self, Write};

use crate::app_config::MultiAppConfig;
use crate::cli::i18n::texts;
use crate::error::AppError;
use crate::store::AppState;

pub fn get_state() -> Result<AppState, AppError> {
    let config = MultiAppConfig::load()?;
    Ok(AppState {
        config: RwLock::new(config),
    })
}

pub fn pause() {
    print!("{} ", texts::press_enter());
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
