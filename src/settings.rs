use config::{Config, File};
use failure;

use std::sync::RwLock;

lazy_static! {
    pub static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::default());
}

pub fn init() -> Result<(), failure::Error> {
    let mut settings = SETTINGS.write().unwrap();

    let mut c = Config::new();
    c.merge(File::with_name("Settings.toml"))?;
    *settings = c.try_into::<Settings>()?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub max_room_capacity: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            max_room_capacity: 0,
        }
    }
}