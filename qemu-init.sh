#!/bin/sh

busybox mount -t devtmpfs none /dev
busybox mount -t proc proc /proc
busybox mount -t sysfs sys /sys

/bin/busybox --install


#busybox modinfo rust_hello.ko
#busybox  insmod rust_hello.ko
#busybox   lsmod

insmod rust_fs.ko
mkdir rustfs
mount -t rustfs none rustfs

cd rustfs

busybox setsid sh -c 'exec sh -l </dev/ttyS0 >/dev/ttyS0 2>&1'
