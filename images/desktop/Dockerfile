FROM ghcr.io/spinupdev/base:latest

ARG DEBIAN_FRONTEND=noninteractive

ENV VNC_PORT=5900 \
    PROXY_PORT=8080 \
    XDG_RUNTIME_DIR=/run/user/1000 \
    WAYLAND_DISPLAY=wayland-0 \
    WLR_BACKENDS=headless \
    WLR_LIBINPUT_NO_DEVICES=1

# No `USER` directive: base already never sets one, and everything below runs
# as the implicit-root default. See base/Dockerfile for why an explicit
# baked Config.User (even "root") breaks depot's initd workload exec.

# ── Wayland + desktop ─────────────────────────────────────────────────────────
# Budgie 10.x has no X11 session anymore (dropped mutter/X11 for its own
# labwc-based Wayland compositor — see /usr/share/wayland-sessions, no
# /usr/share/xsessions entry ships). So this runs labwc headless (WLR_BACKENDS
# above; no real GPU/DRM in the container) with wayvnc as the capture server —
# wayvnc speaks the same RFB protocol x11vnc did, over wlr-screencopy instead
# of XDamage polling, so websockify/noVNC/proxyd's auth all stay untouched.
# Stock Budgie look (no rebrand/theme pass — see git history for the old XFCE
# + Fluent/Bibata rebrand pipeline this replaced).
#
# xdg-desktop-portal(-gnome): file pickers, screenshot/share portals, and
# other GIO-mediated desktop integration a lot of GTK/Electron apps assume
# exists — absent by default in a headless image.
# nautilus/gvfs(-backends): there was no file manager at all before this.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        labwc \
        wayvnc \
        dbus \
        xdg-desktop-portal \
        xdg-desktop-portal-gnome \
        budgie-desktop \
        gnome-terminal \
        nautilus \
        gvfs \
        gvfs-backends \
        fonts-liberation \
        fonts-noto \
        fonts-noto-color-emoji \
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
# Pinned to a commit, not the branch tip: GH Actions' layer cache (type=gha)
# keys on this instruction's literal text, so `--branch zeish-desktop` alone
# would keep serving a stale clone from cache forever after the fork moves —
# bump this SHA whenever zeish-desktop gets new commits that need to ship.
RUN git clone https://github.com/spinupdev/noVNC.git /opt/novnc && \
    git -C /opt/novnc checkout 4defae6 && \
    ln -sf /opt/novnc/vnc.html /opt/novnc/index.html

# ── VS Code (Microsoft's official build, not VSCodium) ───────────────────────
RUN wget -qO- https://packages.microsoft.com/keys/microsoft.asc \
        | gpg --dearmor > /usr/share/keyrings/microsoft.gpg \
    && echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/microsoft.gpg] https://packages.microsoft.com/repos/code stable main" \
        > /etc/apt/sources.list.d/vscode.list \
    && apt-get update && apt-get install -y --no-install-recommends code \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# ── Claude Code + Codex CLIs (official native builds, no Node.js needed) ─────
# Neither Anthropic nor OpenAI ships an official Linux desktop app for these
# (both are Mac/Windows-only) — CLIs only, installed system-wide under
# /usr/local/bin rather than the installer's default ~/.local/bin so every
# shell/user picks them up without PATH changes.
RUN ARCH=$(dpkg --print-architecture) \
    && curl -fsSL https://claude.ai/install.sh | bash \
    && mv /home/user/.local/bin/claude /usr/local/bin/claude \
    && CODEX_ARCH=$([ "$ARCH" = "arm64" ] && echo aarch64 || echo x86_64) \
    && CODEX_VERSION=$(curl -fsSL https://api.github.com/repos/openai/codex/releases/latest | grep -m1 '"tag_name"' | sed -E 's/.*"rust-v([^"]+)".*/\1/') \
    && curl -fsSL "https://github.com/openai/codex/releases/download/rust-v${CODEX_VERSION}/codex-${CODEX_ARCH}-unknown-linux-musl.tar.gz" \
        | tar -xz -C /tmp \
    && mv /tmp/codex-${CODEX_ARCH}-unknown-linux-musl /usr/local/bin/codex \
    && chmod +x /usr/local/bin/codex

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
    && cp /usr/share/applications/org.gnome.Terminal.desktop /home/user/Desktop/ \
    && cp /usr/share/applications/code.desktop /home/user/Desktop/ \
    && cp /usr/share/applications/org.gnome.Nautilus.desktop /home/user/Desktop/ \
    && chmod +x /home/user/Desktop/*.desktop

# ── Dark mode (system-wide dconf default) ────────────────────────────────────
RUN apt-get update && apt-get install -y --no-install-recommends dconf-cli \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY config/dconf/ /etc/dconf/
RUN dconf update

# ── Configs and scripts ───────────────────────────────────────────────────────
COPY config/supervisord.conf /etc/supervisor/supervisord.conf
COPY run.json /etc/run.json
COPY scripts/entrypoint.sh /entrypoint.sh
COPY scripts/websockify-fork.py /usr/local/bin/websockify-fork
RUN chmod +x /entrypoint.sh /usr/local/bin/websockify-fork

# Only chown the dirs just touched as root — NOT `chown -R $HOME`, which
# would copy-up base's multi-GB toolchain into a duplicate layer for a
# no-op ownership change (everything else in $HOME is already user-owned).
RUN mkdir -p /var/log/supervisor \
    && chown -R ${USER}:${USER} ${HOME}/Desktop

EXPOSE ${PROXY_PORT}

# Docker Desktop: entrypoint.sh directly.
# microVM: initd (PID 1) reads /etc/run.json → /entrypoint.sh.
ENTRYPOINT ["/entrypoint.sh"]
