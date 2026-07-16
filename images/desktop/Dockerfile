FROM ghcr.io/spinupdev/base:latest

ARG DEBIAN_FRONTEND=noninteractive

ENV DISPLAY=:1 \
    VNC_PORT=5900 \
    PROXY_PORT=8080 \
    VNC_RESOLUTION=1280x720 \
    VNC_DEPTH=24

# No `USER` directive: base already never sets one, and everything below runs
# as the implicit-root default. See base/Dockerfile for why an explicit
# baked Config.User (even "root") breaks depot's initd workload exec.

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
        # Fluent theme install deps + a standalone dock (XFCE has a panel but
        # no GNOME-style dash; Plank fills that role and reads the same GTK
        # theme, so it comes out visually consistent for free)
        gtk2-engines-murrine \
        sassc \
        plank \
        dconf-cli \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# ── GTK theme + icon theme + cursor theme + wallpapers, rebranded to Zeish ───
# Base look is vinceliuice's Fluent (gtk+icon) and ful1e5's Bibata (cursor) —
# square window buttons, no blur (blur is a GNOME-Shell/Blur-My-Shell-only
# tweak; omitting it is what gives the "normal version" from the repo). Every
# installed dir/index.theme is renamed Fluent/Bibata -> Zeish afterward so
# nothing upstream-branded is user-visible (Appearance dropdowns, About
# panels, etc. all read the directory name / index.theme Name= field).
RUN git clone --depth 1 https://github.com/vinceliuice/Fluent-gtk-theme.git /tmp/fluent-gtk \
    && /tmp/fluent-gtk/install.sh -c dark light -s standard --tweaks square \
    && git clone --depth 1 https://github.com/vinceliuice/Fluent-icon-theme.git /tmp/fluent-icon \
    && /tmp/fluent-icon/install.sh -d /usr/share/icons \
    && git clone --depth 1 --branch Wallpaper https://github.com/vinceliuice/Fluent-gtk-theme.git /tmp/fluent-wallpapers \
    && mkdir -p /usr/share/wallpapers/zeish-fluent \
    && cp /tmp/fluent-wallpapers/wallpaper-1080p/Fluent-mountain-night.png /usr/share/wallpapers/zeish-fluent/zeish-wallpaper-mountain-night.png \
    && cp /tmp/fluent-wallpapers/wallpaper-1080p/Fluent-mountain-day.png /usr/share/wallpapers/zeish-fluent/zeish-wallpaper-mountain-day.png \
    && cp /tmp/fluent-wallpapers/wallpaper-1080p/Fluent-building-night.png /usr/share/wallpapers/zeish-fluent/zeish-wallpaper-building-night.png \
    && cp /tmp/fluent-wallpapers/wallpaper-1080p/wallpaper-default-gradient.png /usr/share/wallpapers/zeish-fluent/zeish-wallpaper-gradient.png \
    && rm -rf /tmp/fluent-gtk /tmp/fluent-icon /tmp/fluent-wallpapers \
    && curl -fsSL https://github.com/ful1e5/Bibata_Cursor/releases/latest/download/Bibata-Modern-Ice.tar.xz \
        | tar -xJ -C /usr/share/icons/ \
    && mv /usr/share/icons/Bibata-Modern-Ice /usr/share/icons/Zeish-Cursor \
    && sed -i -e 's/Bibata-Modern-Ice/Zeish-Cursor/g' -e 's/Bibata/Zeish/g' \
        /usr/share/icons/Zeish-Cursor/index.theme \
    && cd /usr/share/themes \
    && for d in Fluent-*; do mv "$d" "Zeish-${d#Fluent-}"; done \
    && cd /usr/share/icons \
    && for d in Fluent*; do \
        if [ "$d" = "Fluent" ]; then mv "$d" Zeish; else mv "$d" "Zeish-${d#Fluent-}"; fi; \
    done \
    && grep -rl "Fluent" /usr/share/themes/Zeish-* /usr/share/icons/Zeish* --include=index.theme \
        | xargs sed -i 's/Fluent/Zeish/g'

# ── Plank dock, themed to match (system-wide dconf default) ──────────────────
COPY config/dconf/ /etc/dconf/
RUN dconf update

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

# ── VSCodium (VS Code without MS telemetry/branding — fits the rebrand) ──────
RUN curl -fsSL https://gitlab.com/paulcarroty/vscodium-deb-rpm-repo/raw/master/pub.gpg \
        | gpg --dearmor > /usr/share/keyrings/vscodium.gpg \
    && echo "deb [arch=arm64,amd64 signed-by=/usr/share/keyrings/vscodium.gpg] https://download.vscodium.com/debs vscodium main" \
        > /etc/apt/sources.list.d/vscodium.list \
    && apt-get update && apt-get install -y --no-install-recommends codium \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# ── Wine (amd64 only — WineHQ has no arm64 build; Ubuntu's arm64 "wine" route
# is box64 emulation, which is unreliable enough that shipping it broken is
# worse than not shipping it. arm64 sandboxes simply won't have Wine.) ───────
RUN ARCH=$(dpkg --print-architecture) \
    && if [ "$ARCH" = "amd64" ]; then \
        dpkg --add-architecture i386 \
        && curl -fsSL https://dl.winehq.org/wine-builds/winehq.key \
            | gpg --dearmor > /usr/share/keyrings/winehq-archive-keyring.gpg \
        && curl -fsSL https://dl.winehq.org/wine-builds/ubuntu/dists/resolute/winehq-resolute.sources \
            | sed 's#Signed-By: .*#Signed-By: /usr/share/keyrings/winehq-archive-keyring.gpg#' \
            > /etc/apt/sources.list.d/winehq-resolute.sources \
        && apt-get update \
        && apt-get install -y --install-recommends winehq-stable \
        && apt-get install -y --no-install-recommends winetricks \
        && apt-get clean && rm -rf /var/lib/apt/lists/*; \
    fi

# ── Default browser (arch-dependent — see base/Dockerfile's Chrome comment:
# amd64 gets real Google Chrome; arm64's "chrome" is Playwright's automation
# Chromium, not meant to be a human-facing default) ──────────────────────────
RUN ARCH=$(dpkg --print-architecture) \
    && if [ "$ARCH" = "amd64" ]; then \
        update-alternatives --set x-www-browser /usr/bin/google-chrome-stable \
        && update-alternatives --set gnome-www-browser /usr/bin/google-chrome-stable; \
    else \
        update-alternatives --set x-www-browser /usr/bin/firefox \
        && update-alternatives --set gnome-www-browser /usr/bin/firefox; \
    fi

# ── Desktop shortcuts (Chrome on amd64 / Firefox on arm64, Terminal, VSCodium) ─
# Reuses each app's own installed .desktop file rather than hand-authoring
# one, so Exec/Icon stay correct if the app updates.
RUN ARCH=$(dpkg --print-architecture) \
    && mkdir -p /home/user/Desktop \
    && if [ "$ARCH" = "amd64" ]; then \
        cp /usr/share/applications/google-chrome.desktop /home/user/Desktop/; \
    else \
        cp /usr/share/applications/firefox.desktop /home/user/Desktop/; \
    fi \
    && cp /usr/share/applications/xfce4-terminal.desktop /home/user/Desktop/ \
    && cp /usr/share/applications/codium.desktop /home/user/Desktop/ \
    && chmod +x /home/user/Desktop/*.desktop

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

# Only chown the dirs just touched as root — NOT `chown -R $HOME`, which
# would copy-up base's multi-GB toolchain into a duplicate layer for a
# no-op ownership change (everything else in $HOME is already user-owned).
RUN mkdir -p /var/log/supervisor \
    && chown -R ${USER}:${USER} ${HOME}/.config ${HOME}/Desktop

EXPOSE ${PROXY_PORT}

# Docker Desktop: entrypoint.sh directly.
# microVM: initd (PID 1) reads /etc/run.json → /entrypoint.sh.
ENTRYPOINT ["/entrypoint.sh"]
