FROM ubuntu:26.04

ARG DEBIAN_FRONTEND=noninteractive
ARG NODE_MAJOR=26
ARG GO_VERSION=1.24.3

ENV LANG=en_US.UTF-8 \
    TERM=xterm-256color \
    HOME=/home/user \
    PATH="/usr/local/go/bin:/home/user/.local/bin:${PATH}"

# ── Base system ───────────────────────────────────────────────────────────────
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates curl wget gnupg \
        apt-transport-https software-properties-common lsb-release \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# ── Node.js 26 ────────────────────────────────────────────────────────────────
RUN curl -fsSL https://deb.nodesource.com/setup_${NODE_MAJOR}.x | bash - \
    && apt-get install -y nodejs \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# ── GitHub CLI ────────────────────────────────────────────────────────────────
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
        -o /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
        > /etc/apt/sources.list.d/github-cli.list \
    && apt-get update && apt-get install -y gh \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# ── Docker CE ─────────────────────────────────────────────────────────────────
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg \
        | gpg --dearmor -o /usr/share/keyrings/docker.gpg \
    && chmod a+r /usr/share/keyrings/docker.gpg \
    && CODENAME=$(. /etc/os-release && echo "${UBUNTU_CODENAME:-${VERSION_CODENAME}}") \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu ${CODENAME} main" \
        > /etc/apt/sources.list.d/docker.list \
    && apt-get update && apt-get install -y \
        docker-ce docker-ce-cli containerd.io \
        docker-buildx-plugin docker-compose-plugin \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# ── Dev utilities ─────────────────────────────────────────────────────────────
RUN apt-get update && apt-get install -y --no-install-recommends \
        # shell & terminal
        bash bash-completion zsh tmux \
        # editors
        vim neovim \
        # vcs
        git git-lfs \
        # build
        build-essential make cmake pkg-config \
        # python
        python3 python3-pip python3-venv python3-dev \
        # search & navigation
        ripgrep fd-find fzf tree \
        # data & network
        jq netcat-openbsd dnsutils net-tools iputils-ping \
        # system
        sudo locales tzdata procps htop lsof \
        openssh-server openssh-client \
        # file utilities
        unzip zip tar gzip rsync file less man-db \
        # misc
        acl iproute2 kmod udev dbus \
        util-linux rng-tools \
    && locale-gen en_US.UTF-8 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# ── Go ────────────────────────────────────────────────────────────────────────
RUN ARCH=$(dpkg --print-architecture) \
    && curl -fsSL https://go.dev/dl/go${GO_VERSION}.linux-${ARCH}.tar.gz \
        | tar -C /usr/local -xzf -

# ── yq ────────────────────────────────────────────────────────────────────────
RUN ARCH=$(dpkg --print-architecture) \
    && curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_${ARCH} \
        -o /usr/local/bin/yq \
    && chmod +x /usr/local/bin/yq

# ── Python tooling ────────────────────────────────────────────────────────────
RUN pip3 install --no-cache-dir --break-system-packages uv pipx

# ── AI agent CLIs ─────────────────────────────────────────────────────────────
RUN npm install -g \
    @anthropic-ai/claude-code \
    @openai/codex
# TODO: add pi agent once CLI is identified

# ── User setup ────────────────────────────────────────────────────────────────
RUN echo 'tzdata tzdata/Areas select Etc' | debconf-set-selections \
    && echo 'tzdata tzdata/Zones/Etc select UTC' | debconf-set-selections \
    && useradd -u 1000 -m -s /bin/bash -G sudo,docker,users user \
    && echo "user ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/nopasswd \
    && chmod 440 /etc/sudoers.d/nopasswd \
    && echo "" > /etc/machine-id \
    && echo "" > /var/lib/dbus/machine-id \
    && sed -i 's/^AcceptEnv LANG LC_\*$/#AcceptEnv LANG LC_*/' /etc/ssh/sshd_config \
    && echo "root:root" | chpasswd

EXPOSE 22
WORKDIR /workspace
USER user
