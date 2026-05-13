#!/bin/bash
set -e

mkdir -p /var/log/supervisor /run/sshd

exec /usr/bin/supervisord -c /etc/supervisor/supervisord.conf
