#!/bin/bash
set -e

DISK_IMG="/mnt/c/Users/22877/Documents/GitHub/utopia/target/debug/build/utopia_limine-73ee152cde1a5c60/out/utopia-limine.img"
DISK_DIR="/mnt/c/Users/22877/Documents/GitHub/utopia/target/debug/build/utopia_limine-73ee152cde1a5c60/out/disk"

echo "Creating disk image..."
echo "Disk image: $DISK_IMG"
echo "Source directory: $DISK_DIR"

# 创建 64MB 磁盘镜像
dd if=/dev/zero of="$DISK_IMG" bs=1M count=64

# 创建分区表和 FAT32 分区
parted -s "$DISK_IMG" mklabel msdos
parted -s "$DISK_IMG" mkpart primary fat32 1MiB 100%
parted -s "$DISK_IMG" set 1 boot on

# 设置循环设备
LOOP_DEV=$(sudo losetup -f --show -P "$DISK_IMG")
echo "Using loop device: $LOOP_DEV"

# 格式化分区为 FAT32
sudo mkfs.fat -F 32 "${LOOP_DEV}p1"

# 创建挂载点
MOUNT_POINT=$(mktemp -d)
sudo mount "${LOOP_DEV}p1" "$MOUNT_POINT"

# 复制文件到分区
echo "Copying files to disk image..."
sudo cp -r "$DISK_DIR"/* "$MOUNT_POINT/"

# 列出复制的内容
echo "Disk contents:"
ls -la "$MOUNT_POINT/"
ls -la "$MOUNT_POINT/boot/"
ls -la "$MOUNT_POINT/boot/limine/"

# 卸载
sudo umount "$MOUNT_POINT"
rm -rf "$MOUNT_POINT"

# 清理循环设备
sudo losetup -d "$LOOP_DEV"

echo "Disk image with FAT32 filesystem created successfully!"
echo "Image location: $DISK_IMG"
echo ""
echo "Now you need to install the Limine bootloader in Windows PowerShell:"
echo "  .\\limine\\limine-10.8.2-binary\\limine.exe bios-install .\\target\\debug\\build\\utopia_limine-73ee152cde1a5c60\\out\\utopia-limine.img"
