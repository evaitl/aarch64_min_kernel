## Sun Nov 29:

Let's fix the debug version. It seems that the stack pointer isn't set
up at startup. I'd prefer not to do a separate assembly file, so maybe
just a couple of inline assembly instructions at the beginning of `_start`. 


Doesn't work. In debug builds, the compiler uses sp prior to the first
statement. Need to use global_asm.

OK, That works. 

## Sat Nov 28:

Starting on this thing. Basing it on my x86 min kernel. 

There are some instructions
[here](https://lowenware.com/blog/osdev/aarch64-bare-metal-program-in-rust/),
but they don't explain where the addresses come from.  I can't find
the memory map for a qemu aarch64 image from a google search.


    $ qemu-system-aarch64 -cpu cortex-a72  -machine virt,dumpdtb=foo.dtb
    
Creates a dtb. Added "dtc" package to my desktop. 

        $ dtc -I dtb -O dts foo.dtb  | less

Gives a lot to look at. 

Ram: 

       memory@40000000 {
                reg = <0x00 0x40000000 0x00 0x8000000>;
                device_type = "memory";
        };

I can see that RAM is at `0x4000_0000`, but I don't know how the start
address of `0x4008_0000` is picked.

A [pl011](https://tinyurl.com/y4p747zn) compatible uart:

       pl011@9000000 {
                clock-names = "uartclk\0apb_pclk";
                clocks = <0x8000 0x8000>;
                interrupts = <0x00 0x01 0x04>;
                reg = <0x00 0x9000000 0x00 0x1000>;
                compatible = "arm,pl011\0arm,primecell";
        };

In qemu apparently you don't have to set up the divisors or
anything. You can just start scribbling on the data register and it
will be displayed by qemu. 

[Found](https://github.com/qemu/qemu/blob/master/hw/arm/boot.c) it.
Apparently, 64 bit arm kernels have a standard load offset from the
base of system ram of 0x80000.

I guess this is obvious to ARM folks, but nobody told me.


Minimal linker script:

```ld
ENTRY(_start)
SECTIONS
{
    . = 0x40080000;
    .text : { *(.text) *(.rodata) }
    .data : { *(.data) }
    .bss : { *(.bss) }
}
```

Coredump on the compile:

    Caused by: process didn't exit successfully: `rustc --crate-name core --edition=2018 /home/evaitl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=fe51ef0a28433841 -C extra-filename=-fe51ef0a28433841 --out-dir /home/evaitl/git/UGA/CSCI8965/project/aarch64_min_kernel/target/aarch64-xmin/debug/deps --target /home/evaitl/git/UGA/CSCI8965/project/aarch64_min_kernel/aarch64-xmin.json -Z force-unstable-if-unmarked -L dependency=/home/evaitl/git/UGA/CSCI8965/project/aarch64_min_kernel/target/aarch64-xmin/debug/deps -L dependency=/home/evaitl/git/UGA/CSCI8965/project/aarch64_min_kernel/target/debug/deps --cap-lints allow -C link-arg=-Tlink.x` (signal: 11, SIGSEGV: invalid memory reference)

SIGSEGV? Really? Too bad the compiler wasn't written in rust. 

I don't want to play with this, I'm gonna take a break. 


Found the [problem](https://github.com/rust-lang/rust/issues/73677).

I can either:

- Use a (very) old nightly

- Somehow add `RUSTFLAGS="-C llvm-args=-global-isel=false"`

- Or change the target to allow floating point. 


OK. Works, at least in a release build. I am not setting the stack
pointer, which is probably what is killing the debug build. I'll
fiddle with that tomorrow:

    $ cargo build --release
    ....
    $ qemu-system-aarch64 -nographic -m 1024M -cpu cortex-a53 -machine virt -kernel target/aarch64-xmin/release/aarch64_kernel
    Hello world from the kernel
    Hello world from the kernel
    Hello world from the kernel
    Hello world from the kernel
    ....


