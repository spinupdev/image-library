#!/bin/bash

source_img="${1:-workspace}"

#!/bin/bash

build_image_builder() {
  docker build -t rootfs-builder .
}

run_image_builder() {
  mkdir -p $(pwd)/out
  docker run --privileged \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v /images:/images \
    rootfs-builder \
    ${source_img} "${source_img}.ext4"
}


build_image_builder
run_image_builder
