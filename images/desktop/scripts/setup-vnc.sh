#!/bin/bash
# Writes ~/.vnc/passwd for x11vnc auth. Called from entrypoint before supervisord.
set -e

VNC_PASSWD_DIR="${HOME}/.vnc"
VNC_PASSWD_FILE="${VNC_PASSWD_DIR}/passwd"

mkdir -p "${VNC_PASSWD_DIR}"
chmod 700 "${VNC_PASSWD_DIR}"

if [ -z "${VNC_PASSWORD}" ]; then
    echo "ERROR: VNC_PASSWORD is not set." >&2
    exit 1
fi

x11vnc -storepasswd "${VNC_PASSWORD}" "${VNC_PASSWD_FILE}"
chmod 600 "${VNC_PASSWD_FILE}"
