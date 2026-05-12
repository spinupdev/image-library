FROM ubuntu:26.04

ARG DEBIAN_FRONTEND=noninteractive

ENV DISPLAY=:1 \
    VNC_PORT=5900 \
    PROXY_PORT=8080 \
    VNC_RESOLUTION=1280x720 \
    VNC_DEPTH=24 \
    VNC_PASSWORD=changeme \
    USER=user \
    HOME=/home/user

# ── Base system + X11 + desktop ──────────────────────────────────────────────
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
        xfce4-screensaver \
        fonts-liberation \
        fonts-noto \
        fonts-noto-color-emoji \
        x11vnc \
        python3 \
        python3-websockify \
        supervisor \
        curl \
        wget \
        git \
        vim \
        nano \
        htop \
        jq \
        unzip \
        file \
        less \
        rsync \
        ca-certificates \
        gnupg \
        apt-transport-https \
        sudo \
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

# ── initd (PID 1 for microVM) ─────────────────────────────────────────────────
COPY --from=spinupdev/init /init /init
COPY --from=spinupdev/init /init /etc/init
COPY --from=spinupdev/init /init /usr/bin/init

# ── Non-root desktop user ─────────────────────────────────────────────────────
RUN useradd -m -s /bin/bash -G sudo ${USER} && \
    echo "${USER} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/nopasswd && \
    chmod 440 /etc/sudoers.d/nopasswd

# ── Default browser ───────────────────────────────────────────────────────────
RUN update-alternatives --set x-www-browser /usr/bin/firefox && \
    update-alternatives --set gnome-www-browser /usr/bin/firefox

# ── Wallpaper ─────────────────────────────────────────────────────────────────
COPY wallpapers/ /usr/share/wallpapers/zeish/

# ── Configs and scripts ───────────────────────────────────────────────────────
COPY config/supervisord.conf /etc/supervisor/supervisord.conf
COPY config/xfce4/ /etc/skel/.config/xfce4/
COPY run.json /etc/run.json
COPY scripts/entrypoint.sh /entrypoint.sh
COPY scripts/setup-vnc.sh /setup-vnc.sh
RUN chmod +x /entrypoint.sh /setup-vnc.sh

RUN mkdir -p /var/log/supervisor \
    && chown -R ${USER}:${USER} ${HOME}

EXPOSE ${PROXY_PORT}

# Docker Desktop: entrypoint.sh directly.
# microVM: initd (PID 1) reads /etc/run.json → /entrypoint.sh.
ENTRYPOINT ["/entrypoint.sh"]
