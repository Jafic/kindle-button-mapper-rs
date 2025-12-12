#!/bin/sh
# Adjust frontlight brightness via sysfs for finer control
# Usage: brightness.sh <step>
#   step > 0: increase brightness
#   step < 0: decrease brightness
#   Default step: 50 (out of 2010 max)

BRIGHTNESS_FILE="/sys/class/backlight/sgm3756/brightness"
STEP="${1:-50}"
MIN_BRIGHTNESS=0
MAX_BRIGHTNESS=2010

current=$(cat "$BRIGHTNESS_FILE" 2>/dev/null)

# Validate numeric value
case "$current" in
    ''|*[!0-9]*) exit 1 ;;
esac

new=$((current + STEP))

# Clamp to valid range
if [ "$new" -lt "$MIN_BRIGHTNESS" ]; then
    new=$MIN_BRIGHTNESS
elif [ "$new" -gt "$MAX_BRIGHTNESS" ]; then
    new=$MAX_BRIGHTNESS
fi

echo "$new" > "$BRIGHTNESS_FILE"
