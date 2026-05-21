#!/bin/sh
# Name: Button Mapper
# Author: Lucas Zampieri

APP_ID="com.lzampier.mappermanager"
APP_NAME="Button Mapper"
APP_DIR="/mnt/us/kindle-button-mapper/illusion/MapperManager"
BINARY="/mnt/us/kindle-button-mapper/kindle-button-mapper"
CONFIG="/mnt/us/kindle-button-mapper/config.ini"
HELPER_PID="/tmp/kindle-button-mapper-waf.pid"
APPREG_DB="/var/local/appreg.db"

log_msg() { logger -t mappermanager "$1"; }

install_waf() {
    log_msg "Installing MapperManager WAF"
    if [ -f "$APPREG_DB" ]; then
        existing=$(sqlite3 "$APPREG_DB" "SELECT handlerId FROM handlerIds WHERE handlerId='$APP_ID';" 2>/dev/null)
        if [ -z "$existing" ]; then
            log_msg "Registering $APP_ID in appreg.db"
            sqlite3 "$APPREG_DB" <<EOF
INSERT OR IGNORE INTO interfaces (interface) VALUES ('application');
INSERT OR IGNORE INTO handlerIds (handlerId) VALUES ('$APP_ID');
INSERT OR IGNORE INTO associations (handlerId, interface, contentId, defaultAssoc)
    VALUES ('$APP_ID', 'application', 'GL:$APP_ID', 0);
INSERT OR REPLACE INTO properties (handlerId, name, value)
    VALUES ('$APP_ID', 'lipcId', '$APP_ID');
INSERT OR REPLACE INTO properties (handlerId, name, value)
    VALUES ('$APP_ID', 'command', '/usr/bin/mesquite -l $APP_ID -c file://$APP_DIR/');
INSERT OR REPLACE INTO properties (handlerId, name, value)
    VALUES ('$APP_ID', 'supportedOrientation', 'U');
EOF
        fi
    fi
}

start_helper() {
    if [ -f "$HELPER_PID" ] && kill -0 "$(cat "$HELPER_PID")" 2>/dev/null; then
        log_msg "Helper already running"
        return
    fi
    if [ ! -x "$BINARY" ]; then
        log_msg "Binary missing at $BINARY"
        return
    fi
    log_msg "Starting --waf-helper"
    "$BINARY" --waf-helper "$CONFIG" >/var/log/kindle-button-mapper-waf.log 2>&1 &
    echo $! > "$HELPER_PID"
    sleep 1
}

launch_app() {
    log_msg "Launching $APP_NAME"
    cat "$APP_DIR/config.xml" "$APP_DIR/index.html" "$APP_DIR/style.css" "$APP_DIR/script.js" >/dev/null 2>&1
    lipc-set-prop com.lab126.appmgrd start "app://com.lab126.booklet.home"
    sleep 1
    lipc-set-prop com.lab126.appmgrd start "app://$APP_ID"
}

install_waf
start_helper
launch_app
log_msg "MapperManager scriptlet done"
