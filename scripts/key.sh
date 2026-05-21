#!/bin/sh
# Inject a keyboard event into the daemon's virtual uinput keyboard.
# Usage: key.sh KEY_NAME

KEY="$1"
[ -z "$KEY" ] && { echo "Usage: $0 KEY_NAME" >&2; exit 1; }

DEV="$KEY_TARGET_DEV"
[ -z "$DEV" ] && [ -r /var/run/kindle-button-mapper-key-target ] && DEV=$(cat /var/run/kindle-button-mapper-key-target)
[ -z "$DEV" ] && [ -r /etc/kindle-button-mapper-key-target ] && DEV=$(cat /etc/kindle-button-mapper-key-target)

if [ -z "$DEV" ] || [ ! -e "$DEV" ]; then
    echo "key.sh: virtual keyboard not running. Restart the kindle-button-mapper daemon." >&2
    exit 1
fi

evemu-event "$DEV" --type EV_KEY --code "$KEY" --value 1 --sync
evemu-event "$DEV" --type EV_KEY --code "$KEY" --value 0 --sync
