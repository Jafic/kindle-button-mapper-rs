mod config;
mod input;
mod mapper;

use config::Config;
use evdev::InputEventKind;
use input::InputHandler;
use log::{error, info};
use mapper::Mapper;
use nix::sys::signal::{signal, SigHandler, Signal};
use std::env;
use std::process::{self, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();

    // Parse arguments
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!(
            "kindle-button-mapper {} (build {})",
            env!("CARGO_PKG_VERSION"),
            env!("BUILD_SHA")
        );
        return;
    }

    let config_path = if args.len() > 1 {
        &args[1]
    } else {
        "config.ini"
    };

    // Load configuration
    let config = match Config::load(config_path) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load config: {}", e);
            process::exit(1);
        }
    };

    info!(
        "Kindle Button Mapper {} (build {}) starting...",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_SHA")
    );
    info!("Config: debounce={}ms, long_press={}ms, repeat={}ms, grab={}",
        config.debounce_ms, config.long_press_ms, config.repeat_ms, config.grab);

    // Setup signal handlers
    unsafe {
        signal(Signal::SIGINT, SigHandler::Handler(handle_signal)).ok();
        signal(Signal::SIGTERM, SigHandler::Handler(handle_signal)).ok();
    }

    // Create mapper
    let mut mapper = Mapper::new(
        config.mappings,
        config.long_press_mappings,
        config.dpad_mappings,
        config.dpad_longpress_mappings,
        config.trigger_mappings,
        config.trigger_longpress_mappings,
        config.debounce_ms,
        config.long_press_ms,
        config.repeat_ms,
        config.log_buttons,
    );

    // Main loop with reconnection
    while RUNNING.load(Ordering::SeqCst) {
        let handler = InputHandler::new(
            config.device_name.clone(),
            config.device_path.clone(),
            config.grab,
        );

        match handler.open() {
            Ok(mut device) => {
                info!("Device connected");
                // Run on_connect script
                if let Some(ref script) = config.on_connect {
                    info!("Running on_connect script");
                    execute_script(script);
                }
                if let Err(e) = run_event_loop(&mut device, &mut mapper) {
                    error!("Event loop error: {}", e);
                    // Device disconnected - run on_disconnect script
                    if let Some(ref script) = config.on_disconnect {
                        info!("Device disconnected, running on_disconnect script");
                        execute_script(script);
                    }
                }
            }
            Err(e) => {
                error!("Failed to open device: {}", e);
            }
        }

        if RUNNING.load(Ordering::SeqCst) {
            info!("Reconnecting in 1 second...");
            thread::sleep(Duration::from_secs(1));
        }
    }

    info!("Shutting down...");
}

fn run_event_loop(device: &mut evdev::Device, mapper: &mut Mapper) -> Result<(), String> {
    loop {
        if !RUNNING.load(Ordering::SeqCst) {
            return Ok(());
        }

        let events = device.fetch_events()
            .map_err(|e| format!("Read error: {}", e))?;

        for event in events {
            match event.kind() {
                InputEventKind::Key(key) => {
                    match event.value() {
                        1 => mapper.handle_press(key),  // Press
                        2 => mapper.handle_held(key),   // Held/repeat
                        0 => mapper.handle_release(key), // Release
                        _ => {}
                    }
                }
                InputEventKind::AbsAxis(axis) => {
                    let code = axis.0;
                    match code {
                        // D-pad: Hat0X (16) and Hat0Y (17)
                        16 | 17 => mapper.handle_dpad(code, event.value()),
                        // Triggers: Gas (9) = RT, Brake (10) = LT
                        9 | 10 => mapper.handle_trigger(code, event.value()),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

extern "C" fn handle_signal(_: i32) {
    // Exit immediately - fetch_events() blocks so we can't wait for flag check
    std::process::exit(0);
}

fn execute_script(script: &str) {
    match Command::new("/bin/sh").args(["-c", script]).spawn() {
        Ok(mut child) => {
            // Wait for completion (blocking) for disconnect script
            let _ = child.wait();
        }
        Err(e) => {
            error!("Failed to execute '{}': {}", script, e);
        }
    }
}
