use evdev::Device;
use log::{debug, info, warn};
use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

const INPUT_DIR: &str = "/dev/input";

pub struct InputHandler {
    device_name: Option<String>,
    device_path: Option<String>,
    grab: bool,
}

impl InputHandler {
    pub fn new(device_name: Option<String>, device_path: Option<String>, grab: bool) -> Self {
        Self {
            device_name,
            device_path,
            grab,
        }
    }

    pub fn open(&self) -> Result<Device, String> {
        // If path is specified, use it directly
        if let Some(ref path) = self.device_path {
            let path = Path::new(path);

            // If path exists, open it directly
            if path.exists() {
                return self.open_device(path);
            }

            // Path doesn't exist, wait for it with inotify
            info!("Device {} not found, waiting for it to appear...", path.display());
            self.wait_for_path(path)?;
            return self.open_device(path);
        }

        // Otherwise search by name
        let device_name = self.device_name.as_ref()
            .ok_or("No device name or path specified")?;

        // First try to find the device
        if let Some(path) = self.find_device(device_name)? {
            return self.open_device(&path);
        }

        // Device not found, wait for it
        info!("Waiting for device '{}'...", device_name);
        let path = self.wait_for_device(device_name)?;
        self.open_device(&path)
    }

    fn find_device(&self, name: &str) -> Result<Option<std::path::PathBuf>, String> {
        let entries = fs::read_dir(INPUT_DIR)
            .map_err(|e| format!("Cannot open {}: {}", INPUT_DIR, e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            let filename = path.file_name().and_then(OsStr::to_str).unwrap_or("");

            if !filename.starts_with("event") {
                continue;
            }

            match Device::open(&path) {
                Ok(dev) => {
                    let dev_name = dev.name().unwrap_or("");
                    debug!("Device {}: {}", path.display(), dev_name);

                    if dev_name == name {
                        info!("Found device: {} at {}", dev_name, path.display());
                        return Ok(Some(path));
                    }
                }
                Err(e) => {
                    debug!("Cannot open {}: {}", path.display(), e);
                }
            }
        }

        Ok(None)
    }

    fn wait_for_path(&self, target_path: &Path) -> Result<(), String> {
        let inotify = Inotify::init(InitFlags::empty())
            .map_err(|e| format!("inotify_init failed: {}", e))?;

        inotify.add_watch(Path::new(INPUT_DIR), AddWatchFlags::IN_CREATE)
            .map_err(|e| format!("inotify_add_watch failed: {}", e))?;

        let target_name = target_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid device path")?;

        loop {
            let events = inotify.read_events()
                .map_err(|e| format!("inotify read failed: {}", e))?;

            for event in events {
                if let Some(event_name) = &event.name {
                    let event_name_str = event_name.to_string_lossy();
                    if event_name_str == target_name {
                        info!("Device {} appeared", target_path.display());
                        // Give the device a moment to initialize
                        thread::sleep(Duration::from_millis(100));
                        return Ok(());
                    }
                }
            }
        }
    }

    fn wait_for_device(&self, name: &str) -> Result<std::path::PathBuf, String> {
        let inotify = Inotify::init(InitFlags::empty())
            .map_err(|e| format!("inotify_init failed: {}", e))?;

        inotify.add_watch(Path::new(INPUT_DIR), AddWatchFlags::IN_CREATE)
            .map_err(|e| format!("inotify_add_watch failed: {}", e))?;

        loop {
            let events = inotify.read_events()
                .map_err(|e| format!("inotify read failed: {}", e))?;

            for event in events {
                if let Some(event_name) = &event.name {
                    let event_name_str = event_name.to_string_lossy();
                    if event_name_str.starts_with("event") {
                        let path = Path::new(INPUT_DIR).join(&*event_name_str);
                        debug!("New device created: {}", path.display());

                        // Give the device a moment to initialize
                        thread::sleep(Duration::from_millis(100));

                        if let Ok(dev) = Device::open(&path) {
                            let dev_name = dev.name().unwrap_or("");
                            if dev_name == name {
                                info!("Found device: {} at {}", dev_name, path.display());
                                return Ok(path);
                            }
                        }
                    }
                }
            }
        }
    }

    fn open_device(&self, path: &Path) -> Result<Device, String> {
        let mut device = Device::open(path)
            .map_err(|e| format!("Cannot open {}: {}", path.display(), e))?;

        if self.grab {
            if let Err(e) = device.grab() {
                warn!("Cannot grab device: {}, continuing without exclusive access", e);
            }
        }

        info!("Reading events from {}", path.display());
        Ok(device)
    }
}
