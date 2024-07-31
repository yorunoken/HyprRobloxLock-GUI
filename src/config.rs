use hyprland::data::Monitor;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub screen_width: i64,
    pub screen_height: i64,
    pub monitor_x: i64,
    pub monitor_y: i64,
    pub edge_offset: i64,
}

impl Config {
    pub fn new(edge_offset: i64, monitor: &Monitor) -> Result<Self, Box<dyn Error>> {
        let screen_width = monitor.width as i64;
        let screen_height = monitor.height as i64;
        let monitor_x = monitor.x as i64;
        let monitor_y = monitor.y as i64;

        Ok(Config {
            edge_offset,
            screen_width,
            screen_height,
            monitor_x,
            monitor_y,
        })
    }
}
