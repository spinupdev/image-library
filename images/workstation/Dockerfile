FROM ghcr.io/spinupdev/base:latest

ARG DEBIAN_FRONTEND=noninteractive

USER root

# ── SSH access (headless workstation only) ────────────────────────────────────
RUN apt-get update && apt-get install -y --no-install-recommends \
        openssh-server \
    && apt-get clean && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /var/run/sshd \
    && sed -i 's/^AcceptEnv LANG LC_\*$/#AcceptEnv LANG LC_*/' /etc/ssh/sshd_config

EXPOSE 22
WORKDIR /workspace
USER user
