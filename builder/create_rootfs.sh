#!/bin/bash

rootfs_mount="/tmp/rootfs"
output_dir="/images/rootfs"
source_container_img="${1:-workspace}"
# output_img="${2:-'disk.ext4'}"
img_file="${output_dir}/${source_container_img}.ext4"

prepare_out() {
  mkdir -p ${output_dir}
  rm -rf ${img_file}
}

echo_bold() {
  local content
  content=$1
  echo "---> ${content}"
}

empty_ext4_fs() {
  echo_bold "Creating empty ext4 filesystem for ${img_file}"
  # dd if=/dev/zero of=${img_file} bs=1M count=50
  truncate -s 5G ${img_file}
  mkfs.ext4 ${img_file}
}

mount_image() {
  # Create temp mount directory
  rm -rf ${rootfs_mount}
  mkdir -p ${rootfs_mount}

  # Mount the image as a loop device
  echo_bold "Mounting rootfs image to ${rootfs_mount}"
  mount -o loop ${img_file} ${rootfs_mount}
}

unmount_image() {
  umount ${rootfs_mount}
}

create_root_fs_from_container() {
  local ctr_img
  ctr_img=$1
  tmp_rootfs="/tmp/rootfs.tar"

  echo_bold "Building rootfs from docker image: ${ctr_img}"
  container_id=$(docker create ${ctr_img})

  echo_bold "Exporting docker image content to ${tmp_rootfs}"
  docker export ${container_id} > ${tmp_rootfs}

  # rm -rf ${rootfs_mount}
  tar -C ${rootfs_mount} -xf ${tmp_rootfs}
  cp /images/init/init ${rootfs_mount}/etc/init
  cp /images/init/init ${rootfs_mount}/sbin/init
  cp /images/init/init ${rootfs_mount}/init
  cp /images/init/init ${rootfs_mount}/goinit
  echo_bold "Copied INIT"
     
  rm -rf ${tmp_rootfs}
}

copy_init_rootfs() {
  # id=$(docker create spinupdev/init)
  # docker cp $id:/init ${rootfs_mount}/hinit
  # docker rm -v ${id}
  cp /images/init/init ${rootfs_mount}/etc/init
  echo_bold "Copied INIT"
}

resize_image_min_size() {
  resize2fs -M ${img_file}
}

check_img_fs() {
  e2fsck -y -f ${img_file}
}

echo_bold "Received args $1, $2"
prepare_out
empty_ext4_fs
mount_image
create_root_fs_from_container ${source_container_img}
copy_init_rootfs
# copy_test_application
unmount_image
check_img_fs
# resize_image_min_size
