use configparser::ini::Ini;
use evdev::Key;
use std::collections::HashMap;
use std::path::Path;

/// D-pad directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DpadDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Trigger buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trigger {
    LT, // Left Trigger (Brake, code 10)
    RT, // Right Trigger (Gas, code 9)
}

#[derive(Debug)]
pub struct Config {
    pub device_name: Option<String>,
    pub device_path: Option<String>,
    pub grab: bool,
    pub debounce_ms: u64,
    pub long_press_ms: u64,
    pub repeat_ms: u64,
    pub log_buttons: bool,
    pub on_connect: Option<String>,
    pub on_disconnect: Option<String>,
    pub mappings: HashMap<Key, String>,
    pub long_press_mappings: HashMap<Key, String>,
    pub dpad_mappings: HashMap<DpadDirection, String>,
    pub dpad_longpress_mappings: HashMap<DpadDirection, String>,
    pub trigger_mappings: HashMap<Trigger, String>,
    pub trigger_longpress_mappings: HashMap<Trigger, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device_name: None,
            device_path: None,
            grab: false,
            debounce_ms: 200,
            long_press_ms: 500,
            repeat_ms: 100,
            log_buttons: false,
            on_connect: None,
            on_disconnect: None,
            mappings: HashMap::new(),
            long_press_mappings: HashMap::new(),
            dpad_mappings: HashMap::new(),
            dpad_longpress_mappings: HashMap::new(),
            trigger_mappings: HashMap::new(),
            trigger_longpress_mappings: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut ini = Ini::new();
        ini.load(path).map_err(|e| format!("Failed to load config: {}", e))?;

        let mut config = Config::default();

        // [device] section
        if let Some(name) = ini.get("device", "name") {
            config.device_name = Some(name);
        }
        if let Some(path) = ini.get("device", "path") {
            config.device_path = Some(path);
        }
        if let Some(grab) = ini.getbool("device", "grab").ok().flatten() {
            config.grab = grab;
        }

        // [settings] section
        if let Some(debounce) = ini.getuint("settings", "debounce_ms").ok().flatten() {
            config.debounce_ms = debounce;
        }
        if let Some(long_press) = ini.getuint("settings", "long_press_ms").ok().flatten() {
            config.long_press_ms = long_press;
        }
        if let Some(repeat) = ini.getuint("settings", "repeat_ms").ok().flatten() {
            config.repeat_ms = repeat;
        }
        if let Some(log) = ini.getbool("settings", "log_buttons").ok().flatten() {
            config.log_buttons = log;
        }
        if let Some(script) = ini.get("settings", "on_connect") {
            if !script.is_empty() {
                config.on_connect = Some(script);
            }
        }
        if let Some(script) = ini.get("settings", "on_disconnect") {
            if !script.is_empty() {
                config.on_disconnect = Some(script);
            }
        }

        // [buttons] section
        if let Some(buttons) = ini.get_map_ref().get("buttons") {
            for (key_str, script) in buttons {
                if let (Some(key), Some(script)) = (parse_key(key_str), script) {
                    config.mappings.insert(key, script.clone());
                }
            }
        }

        // [longpress] section
        if let Some(buttons) = ini.get_map_ref().get("longpress") {
            for (key_str, script) in buttons {
                if let (Some(key), Some(script)) = (parse_key(key_str), script) {
                    config.long_press_mappings.insert(key, script.clone());
                }
            }
        }

        // [dpad] section
        if let Some(dpad) = ini.get_map_ref().get("dpad") {
            for (dir_str, script) in dpad {
                if let (Some(dir), Some(script)) = (parse_dpad_direction(dir_str), script) {
                    config.dpad_mappings.insert(dir, script.clone());
                }
            }
        }

        // [dpad_longpress] section
        if let Some(dpad) = ini.get_map_ref().get("dpad_longpress") {
            for (dir_str, script) in dpad {
                if let (Some(dir), Some(script)) = (parse_dpad_direction(dir_str), script) {
                    config.dpad_longpress_mappings.insert(dir, script.clone());
                }
            }
        }

        // [triggers] section
        if let Some(triggers) = ini.get_map_ref().get("triggers") {
            for (trigger_str, script) in triggers {
                if let (Some(trigger), Some(script)) = (parse_trigger(trigger_str), script) {
                    config.trigger_mappings.insert(trigger, script.clone());
                }
            }
        }

        // [triggers_longpress] section
        if let Some(triggers) = ini.get_map_ref().get("triggers_longpress") {
            for (trigger_str, script) in triggers {
                if let (Some(trigger), Some(script)) = (parse_trigger(trigger_str), script) {
                    config.trigger_longpress_mappings.insert(trigger, script.clone());
                }
            }
        }

        if config.device_name.is_none() && config.device_path.is_none() {
            return Err("Either device.name or device.path must be specified".to_string());
        }

        Ok(config)
    }
}

fn parse_key(s: &str) -> Option<Key> {
    // Try parsing as decimal
    if let Ok(code) = s.parse::<u16>() {
        return Some(Key::new(code));
    }

    // Try parsing as hex (0x prefix)
    if let Some(hex) = s.strip_prefix("0x") {
        if let Ok(code) = u16::from_str_radix(hex, 16) {
            return Some(Key::new(code));
        }
    }

    // Try parsing as named key
    match s.to_uppercase().as_str() {
        "KEY_ESC" => Some(Key::KEY_ESC),
        "KEY_1" => Some(Key::KEY_1),
        "KEY_2" => Some(Key::KEY_2),
        "KEY_3" => Some(Key::KEY_3),
        "KEY_4" => Some(Key::KEY_4),
        "KEY_5" => Some(Key::KEY_5),
        "KEY_6" => Some(Key::KEY_6),
        "KEY_7" => Some(Key::KEY_7),
        "KEY_8" => Some(Key::KEY_8),
        "KEY_9" => Some(Key::KEY_9),
        "KEY_0" => Some(Key::KEY_0),
        "KEY_ENTER" => Some(Key::KEY_ENTER),
        "KEY_SPACE" => Some(Key::KEY_SPACE),
        "KEY_UP" => Some(Key::KEY_UP),
        "KEY_DOWN" => Some(Key::KEY_DOWN),
        "KEY_LEFT" => Some(Key::KEY_LEFT),
        "KEY_RIGHT" => Some(Key::KEY_RIGHT),
        "KEY_HOME" => Some(Key::KEY_HOME),
        "KEY_END" => Some(Key::KEY_END),
        "KEY_PAGEUP" => Some(Key::KEY_PAGEUP),
        "KEY_PAGEDOWN" => Some(Key::KEY_PAGEDOWN),
        "KEY_VOLUMEUP" => Some(Key::KEY_VOLUMEUP),
        "KEY_VOLUMEDOWN" => Some(Key::KEY_VOLUMEDOWN),
        "KEY_POWER" => Some(Key::KEY_POWER),
        "KEY_BACK" => Some(Key::KEY_BACK),
        "KEY_MENU" => Some(Key::KEY_MENU),
        "KEY_F1" => Some(Key::KEY_F1),
        "KEY_F2" => Some(Key::KEY_F2),
        "KEY_F3" => Some(Key::KEY_F3),
        "KEY_F4" => Some(Key::KEY_F4),
        "KEY_F5" => Some(Key::KEY_F5),
        _ => None,
    }
}

fn parse_dpad_direction(s: &str) -> Option<DpadDirection> {
    match s.to_lowercase().as_str() {
        "up" => Some(DpadDirection::Up),
        "down" => Some(DpadDirection::Down),
        "left" => Some(DpadDirection::Left),
        "right" => Some(DpadDirection::Right),
        _ => None,
    }
}

fn parse_trigger(s: &str) -> Option<Trigger> {
    match s.to_lowercase().as_str() {
        "lt" | "left" => Some(Trigger::LT),
        "rt" | "right" => Some(Trigger::RT),
        _ => None,
    }
}
