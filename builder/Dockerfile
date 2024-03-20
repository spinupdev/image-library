FROM ubuntu:22.04

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
  curl \
  build-essential \
  debootstrap \
  docker.io \
  git \
  && rm -rf /var/lib/apt/lists/*

COPY create_rootfs.sh create_rootfs.sh

VOLUME /out

ENTRYPOINT ["./create_rootfs.sh"]
