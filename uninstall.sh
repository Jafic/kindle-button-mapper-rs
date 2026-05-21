#!/bin/sh
set -e

INSTALL_DIR=/mnt/us/kindle-button-mapper
APPREG_DB=/var/local/appreg.db
APP_ID=com.lzampier.mappermanager

/etc/init.d/kindle-button-mapper stop 2>/dev/null || true

# Helper + WAF mesquite — release any handles still open on the install dir.
lipc-set-prop com.lab126.appmgrd start app://com.lab126.booklet.home 2>/dev/null || true
pkill -TERM -f mesquite.*mappermanager 2>/dev/null || true
kill "$(cat /tmp/kindle-button-mapper-waf.pid 2>/dev/null)" 2>/dev/null || true
pkill -f waf-helper 2>/dev/null || true
sleep 1

/usr/sbin/mntroot rw

rm -f /etc/init.d/kindle-button-mapper

if [ -f "$APPREG_DB" ]; then
    sqlite3 "$APPREG_DB" <<EOF
DELETE FROM properties WHERE handlerId='$APP_ID';
DELETE FROM associations WHERE handlerId='$APP_ID';
DELETE FROM handlerIds WHERE handlerId='$APP_ID';
EOF
fi

/usr/sbin/mntroot ro || true

rm -rf "$INSTALL_DIR"
rm -f /mnt/us/documents/MapperManager.sh

echo "Uninstalled."
