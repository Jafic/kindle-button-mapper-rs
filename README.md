# Kindle Button Mapper

A Rust-based Linux input device event mapper for Kindle e-readers. Maps button presses from input devices to shell scripts.

## Features

- Map buttons to shell scripts
- Long press support with separate actions
- Auto-repeat when buttons are held
- Debouncing to prevent double-triggers
- Auto-reconnect on device disconnect
- Optional exclusive device grab

## Building

```bash
cargo build --release
```

### Cross-Compilation (ARM)

```bash
cargo build --release --target armv7-unknown-linux-gnueabihf
```

## Usage

```bash
kindle-button-mapper /path/to/config.ini
```

Enable debug logging with `RUST_LOG=debug`.

## Configuration

INI format configuration file:

```ini
[device]
name = "Device Name"
# path = /dev/input/event2
grab = true

[settings]
debounce_ms = 200
long_press_ms = 500
repeat_ms = 100
log_buttons = true
on_connect = /path/to/script.sh
on_disconnect = /path/to/script.sh

[buttons]
# button_code = /path/to/script.sh

[longpress]
# button_code = /path/to/script.sh

[dpad]
# up/down/left/right = /path/to/script.sh

[dpad_longpress]
# up/down/left/right = /path/to/script.sh

[triggers]
# lt/rt = /path/to/script.sh

[triggers_longpress]
# lt/rt = /path/to/script.sh
```

Use `log_buttons = true` to discover button codes for your device.

## License

MIT
