# rou2exOS Rusted Edition

A second iteration of the RoureXOS operating system, rewritten in Rust.

+ [Original RoureXOS (a blog post)](https://krusty.space/projects/rourexos/)
+ [rou2exOS Rusted Edition (a blog post)](https://blog.vxn.dev/rou2exos-rusted-edition)

![rou2exOS startup](https://blog.vxn.dev/images/posts/rou2exos/cover.webp)

To run the OS, you can use the attached ISO image from any Release, and run it in QEMU emulator. The system was also tested on the x86_64 baremetal (booted from the USB flash disk).

## How to build and run

```shell
# install Rust and its dependencies
make init

# make sure you have `xorriso`, `net-tools` and `grub2-tools` (or just grub-tools) 
# installed (Linux)
dnf install xorriso net-tools grub2-tools qemu qemu-common qemu-system-x86

# compile the kernel and stage2 bootloader, link it into an ELF binary and bake into an ISO
# image with GRUB stage1 bootloader
make build

# run the QEMU emulation with ISO image
make run_iso

# create a floppy image and attach it to virtual machine (will enable filesystem-related features)
# please do note that the floppy image is overwritten every time you hit this target
make run_iso_floppy

# (alternative) run the kernel exclusively only (needs the `bootloader` 
# dependency in Cargo.toml to be added)
cargo bootimage
make run
```

## How to test ICMP/SLIP 

Start a virtual machine to receive the `pty` handle:

```
make run_iso

char device redirected to /dev/pts/3 (label serial0)
```

Listen for SLIP packets and create a `sl0` interface:

```
sudo slattach -L -p slip -s 115200 /dev/pts/3
sudo ifconfig sl0 192.168.3.1 pointopoint 192.168.3.2 up
```

Catch packets using `tcpdump`:

```
sudo tcpdump -i sl0
```

Run the `response` command in the system shell to handle ICMP
```rou2exOS
response
```

Now you should be able to ping the machine from your machine
```
ping 192.168.3.2
```

