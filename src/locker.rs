use crate::config::Config;
use crate::dotool::DoTool;

use hyprland::data::{CursorPosition, Monitor};
use hyprland::shared::{HyprData, HyprDataActive};

use std::cmp::{max, min};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

#[derive(Clone)]
pub struct CursorLocker {
    pub is_on: Arc<AtomicBool>,
    pub timer_ran_out: Arc<AtomicBool>,
}

impl CursorLocker {
    pub fn new() -> Self {
        CursorLocker {
            is_on: Arc::new(AtomicBool::new(false)),
            timer_ran_out: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self, edge_offset: i64) {
        let is_on = Arc::clone(&self.is_on);

        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(5));

            let mut dotool = DoTool::new().unwrap();
            let monitor = Monitor::get_active().unwrap();
            let config = Config::new(edge_offset, &monitor).unwrap();

            while is_on.load(Ordering::SeqCst) {
                let cursor_pos = CursorPosition::get().expect("Couldn't get cursor position.");
                let cur_x = cursor_pos.x;
                let cur_y = cursor_pos.y;

                // Calculate relative cursor position
                let relative_x = cur_x - config.monitor_x;
                let relative_y = cur_y - config.monitor_y;

                let clamped_x = min(max(relative_x, 0), config.screen_width);
                let clamped_y = min(max(relative_y, 0), config.screen_height);

                println!("");
                println!("clamped_x: {}", clamped_x);
                println!("clamped_y: {}", clamped_y);
                println!("cur_x: {}", cur_x);
                println!("cur_y: {}", cur_y);

                if clamped_x <= config.edge_offset {
                    println!("outside of clamp_x border (left)");
                    let y_percent = clamped_y as f32 / config.screen_height as f32;
                    dotool
                        .write(format!("mouseto 0.8 {}", y_percent).as_str())
                        .unwrap()
                }
                if clamped_x + config.edge_offset >= config.screen_width {
                    println!("outside of clamp_x border (right)");
                    let y_percent = clamped_y as f32 / config.screen_height as f32;
                    dotool
                        .write(format!("mouseto 0.2 {}", y_percent).as_str())
                        .unwrap()
                }

                if clamped_y <= config.edge_offset {
                    println!("outside of clamp_y border (up)");
                    let x_percent = clamped_x as f32 / config.screen_width as f32;
                    dotool
                        .write(format!("mouseto {} 0.8", x_percent).as_str())
                        .unwrap()
                }

                if clamped_y + config.edge_offset >= config.screen_height {
                    println!("outside of clamp_y border (down)");
                    let x_percent = clamped_x as f32 / config.screen_width as f32;
                    dotool
                        .write(format!("mouseto {} 0.2", x_percent).as_str())
                        .unwrap()
                }
            }
        });
    }
}
