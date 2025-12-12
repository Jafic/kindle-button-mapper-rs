#!/bin/sh
# Toggle frontlight between min and max brightness via sysfs

BRIGHTNESS_FILE="/sys/class/backlight/sgm3756/brightness"
MAX_BRIGHTNESS=2010
MIN_BRIGHTNESS=0
MIDPOINT=1005

current_brightness=$(cat "$BRIGHTNESS_FILE" 2>/dev/null)

# Validate numeric value
case "$current_brightness" in
    ''|*[!0-9]*) exit 1 ;;
esac

if [ "$current_brightness" -gt "$MIDPOINT" ]; then
    new_brightness=$MIN_BRIGHTNESS
else
    new_brightness=$MAX_BRIGHTNESS
fi

echo "$new_brightness" > "$BRIGHTNESS_FILE"
