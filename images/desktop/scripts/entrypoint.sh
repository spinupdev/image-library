#!/bin/bash
set -e

# Generate a random VNC password if none is set and auth is disabled
if [ -z "${VNC_PASSWORD}" ]; then
    export VNC_PASSWORD="$(head -c 16 /dev/urandom | base64 | tr -dc 'a-zA-Z0-9' | head -c 16)"
fi

# Write VNC passwd file as the desktop user
su -c "/setup-vnc.sh" "${USER}"

mkdir -p /var/log/supervisor /run/sshd

exec /usr/bin/supervisord -c /etc/supervisor/supervisord.conf
