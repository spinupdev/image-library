#!/bin/bash
set -e

mkdir -p /var/log/supervisor /run/sshd /run/dbus
# Budgie's session manager launches its autostart apps (wm, panel, ...) over
# ICE/XSMP, which refuses to create this dir itself unless euid==0 — normally
# a distro's systemd-tmpfiles does this at boot; nothing here does, so without
# it every ICE-managed autostart silently fails and the session is a bare root
# window with nothing drawn.
mkdir -p -m 1777 /tmp/.ICE-unix /tmp/.X11-unix

# XDG_RUNTIME_DIR for the Wayland session (labwc's socket, wayvnc's control
# socket, ...) — normally pam_systemd creates this at login; nothing here
# does, and /run/user itself isn't writable by `user`, so seed it as root.
mkdir -p -m 0700 "${XDG_RUNTIME_DIR}"
chown "${USER}:${USER}" "${XDG_RUNTIME_DIR}"

exec /usr/bin/supervisord -c /etc/supervisor/supervisord.conf
