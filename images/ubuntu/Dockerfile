FROM ubuntu:26.04

COPY bash_profile /etc/skel/.bash_profile

RUN : \
    && echo 'tzdata tzdata/Areas select Etc' | debconf-set-selections \
    && echo 'tzdata tzdata/Zones/Etc select UTC' | debconf-set-selections \
    && apt-get update -y \
    && DEBIAN_FRONTEND="noninteractive" apt-get install -y \
        tzdata \
        git \
        util-linux \
        curl \
        iputils-ping \
        openssh-server \
        locales \
        sudo \
        procps \
        wget \
        unzip \
        mc \
        vim \
        net-tools \
        ca-certificates \
        curl \
        software-properties-common \
        bash-completion \
        man \
        acl \
        dbus \
        kmod \
        iproute2 \
        rng-tools \
        udev \
        vim-tiny \
    && :


RUN : \
    && echo "%sudo ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers \
    && useradd -u 1000 -G users,sudo,root -d /home/user --shell /bin/bash -m user \
    && usermod -p "*" user \
    && :


ENV LANG en_US.UTF-8
ENV TERM xterm-256color

EXPOSE 8000
WORKDIR /projects

USER user

COPY id_ed25519.pub /home/user/.ssh/authorized_keys

RUN sudo chmod 644 /home/user/.ssh/authorized_keys

RUN sudo locale-gen en_US.UTF-8
# The following instructions set the right
# permissions and scripts to allow the container
# to be run by an arbitrary user (i.e. a user
# that doesn't already exist in /etc/passwd)
ENV HOME /home/user
RUN for f in "/home/user" "/etc/passwd" "/etc/group" "/projects"; do\
        sudo chgrp -R 0 ${f} && \
        sudo chmod -R g+rwX ${f}; \
    done && \
    # Generate passwd.template \
    cat /etc/passwd | \
    sed s#user:x.*#user:x:\${USER_ID}:\${GROUP_ID}::\${HOME}:/bin/bash#g \
    > /home/user/passwd.template && \
    # Generate group.template \
    cat /etc/group | \
    sed s#root:x:0:#root:x:0:0,\${USER_ID}:#g \
    > /home/user/group.template

USER root

# Create the following files, but unset them
RUN echo "" > /etc/machine-id && echo "" > /var/lib/dbus/machine-id

# This container image doesn't have locales installed. Disable forwarding the
# user locale env variables or we get warnings such as:
#  bash: warning: setlocale: LC_ALL: cannot change locale
RUN sed -i -e 's/^AcceptEnv LANG LC_\*$/#AcceptEnv LANG LC_*/' /etc/ssh/sshd_config

# Set the root password to root when logging in through the VM's ttyS0 console
RUN echo "root:root" | chpasswd


COPY --from=spinupdev/init /init /init
COPY --from=spinupdev/init /init /etc/init
COPY --from=spinupdev/init /init /usr/bin/init
RUN apt-get -y autoremove && \
    apt-get -y clean && \
    rm -rf /var/lib/apt/lists/*

# ENTRYPOINT ["/opt/vscode/server.sh", "--connectionToken", "secret"]
