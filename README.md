# aarch64_min_kernel
Bare metal minimum binary for aarch64

Instructions:

    $ cargo build
    $ qemu-system-aarch64 -nographic -cpu cortex-a72 -machine virt -kernel target/aarch64-xmin/debug/aarch64_kernel

