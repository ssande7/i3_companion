pub mod keybinding;
pub mod traits;
pub mod pipe_sender;
pub mod layout_tracker;
pub mod output_tracker;
pub mod ws_history;

use std::time::Duration;
use traits::OnEvent;
use ws_history::{WSHistory, WSHistoryConfig};
use layout_tracker::{LayoutTracker, LayoutTrackerConfig};
use output_tracker::{OutputTracker, OutputTrackerConfig};

pub struct Config {
    pub connection_timeout: Duration,  // secs
    pub connection_interval: Duration, // millis
    pub ws_history: Option<WSHistoryConfig>,
    pub layout_tracker: Option<LayoutTrackerConfig>,
    pub output_tracker: Option<OutputTrackerConfig>,
}

impl Config {
    pub fn new() -> Self {
        // TODO: read from command line args or .config/i3-companion/config
        Self {
            connection_timeout: Duration::from_secs(3),
            connection_interval: Duration::from_millis(10),
            ws_history: None,
            layout_tracker: None,
            output_tracker: None,
        }
    }

    // Send trait not required right now, but keeping for future parallization
    pub fn get_handlers(&self) -> Vec<Box<dyn OnEvent + Send>> {
        let mut handlers = Vec::<Box<dyn OnEvent + Send>>::new();
        if let Some(config) = &self.ws_history {
            let wshist = Box::new(WSHistory::from(config));
            handlers.push(wshist);
        }
        if let Some(config) = &self.layout_tracker {
            handlers.push(Box::new(LayoutTracker::from(config)));
        }
        if let Some(config) = &self.output_tracker {
            handlers.push(Box::new(OutputTracker::from(config)));
        }
        handlers
    }
}
