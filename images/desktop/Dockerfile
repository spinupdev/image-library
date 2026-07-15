FROM ghcr.io/spinupdev/base:latest

ARG DEBIAN_FRONTEND=noninteractive

ENV DISPLAY=:1 \
    VNC_PORT=5900 \
    PROXY_PORT=8080 \
    VNC_RESOLUTION=1280x720 \
    VNC_DEPTH=24

USER root

# ── X11 + desktop ─────────────────────────────────────────────────────────────
# No screensaver/lock package installed — nothing to disable, nothing to lock
# the session after idle.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        xvfb \
        xserver-xorg-core \
        x11-xserver-utils \
        x11-utils \
        dbus-x11 \
        libx11-xcb1 \
        xfce4 \
        xfce4-terminal \
        xfce4-goodies \
        fonts-liberation \
        fonts-noto \
        fonts-noto-color-emoji \
        x11vnc \
        python3-websockify \
        supervisor \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# ── Firefox (proper .deb from Mozilla, not snap) ─────────────────────────────
RUN wget -q https://packages.mozilla.org/apt/repo-signing-key.gpg -O- | \
        gpg --dearmor > /usr/share/keyrings/mozilla.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/mozilla.gpg] https://packages.mozilla.org/apt mozilla main" \
        > /etc/apt/sources.list.d/mozilla.list && \
    printf 'Package: *\nPin: origin packages.mozilla.org\nPin-Priority: 1000\n' \
        > /etc/apt/preferences.d/mozilla && \
    apt-get update && \
    apt-get install -y --no-install-recommends firefox && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# ── noVNC (spinupdev fork, zeish-desktop branch: stripped UI, auto-connect) ──
RUN git clone --depth 1 --branch zeish-desktop \
    https://github.com/spinupdev/noVNC.git /opt/novnc && \
    ln -sf /opt/novnc/vnc.html /opt/novnc/index.html

# ── Default browser ───────────────────────────────────────────────────────────
RUN update-alternatives --set x-www-browser /usr/bin/firefox && \
    update-alternatives --set gnome-www-browser /usr/bin/firefox

# ── Wallpapers ─────────────────────────────────────────────────────────────────
COPY wallpapers/ /usr/share/wallpapers/zeish/

# ── Configs and scripts ───────────────────────────────────────────────────────
# xfce4 config goes straight into the user's real HOME (base already created
# `user`) — NOT /etc/skel, which only seeds accounts created *after* this COPY
# and was never read since the user already existed.
COPY config/supervisord.conf /etc/supervisor/supervisord.conf
COPY config/xfce4/ /home/user/.config/xfce4/
COPY run.json /etc/run.json
COPY scripts/entrypoint.sh /entrypoint.sh
COPY scripts/websockify-fork.py /usr/local/bin/websockify-fork
RUN chmod +x /entrypoint.sh /usr/local/bin/websockify-fork

# Only chown the config dir just copied in as root — NOT `chown -R $HOME`,
# which would copy-up base's multi-GB toolchain into a duplicate layer for a
# no-op ownership change (everything else in $HOME is already user-owned).
RUN mkdir -p /var/log/supervisor \
    && chown -R ${USER}:${USER} ${HOME}/.config

EXPOSE ${PROXY_PORT}

# Docker Desktop: entrypoint.sh directly.
# microVM: initd (PID 1) reads /etc/run.json → /entrypoint.sh.
ENTRYPOINT ["/entrypoint.sh"]
