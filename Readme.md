Minimal build for VU+ DUO2 mipsel-unknown-linux-gnu with Rust on Windows
==============================================================================

To save you some hours i describe the process how i build a hello world without
ending up with `Illegal instruction` errors on the **BCM7425B2 STB platform**.
And irc is no help. No clue. I thank #Linuxger Stuttgart for the support.

Target Specs

    root@vuduo2:~# cat /proc/cpuinfo
    system type             : BCM7425B2 STB platform
    machine                 : Unknown
    processor               : 0
    cpu model               : Broadcom BMIPS5000 V1.1  FPU V0.1
    BogoMIPS                : 864.25
    cpu MHz                 : 1305.097
    wait instruction        : yes
    microsecond timers      : yes
    tlb_entries             : 64
    extra interrupt vector  : yes
    hardware watchpoint     : no
    isa                     : mips1 mips2 mips32r1
    ASEs implemented        :
    shadow register sets    : 1
    kscratch registers      : 0
    core                    : 0
    VCED exceptions         : not available
    VCEI exceptions         : not available
    
    processor               : 1
    cpu model               : Broadcom BMIPS5000 V1.1  FPU V0.1
    BogoMIPS                : 655.36
    cpu MHz                 : 1305.097
    wait instruction        : yes
    microsecond timers      : yes
    tlb_entries             : 64
    extra interrupt vector  : yes
    hardware watchpoint     : no
    isa                     : mips1 mips2 mips32r1
    ASEs implemented        :
    shadow register sets    : 1
    kscratch registers      : 0
    core                    : 0
    VCED exceptions         : not available
    VCEI exceptions         : not available

## What you need

To compile for the target i used **Codescape GNU Tools 2018.09-03 Binaries** 
    https://codescape.mips.com/components/toolchain/2018.09-03/downloads.html
    installed in `C:\Toolchain\MIPS\mips-2016.05`.

Rust components used: 

    E:\Projects\Rust\vuduo2\hello-vuduo>rustup show
    Default host: x86_64-pc-windows-gnu
    
    installed targets for active tool chain
    --------------------------------------

    mipsel-unknown-linux-gnu
    x86_64-pc-windows-gnu
    
    active tool chain
    ----------------
    
    nightly-x86_64-pc-windows-gnu (default)
    rustc 1.35.0-nightly (719b0d984 2019-03-13)

Install *"The sysroot manager that lets you build and customize `std`"* 
[xargo](https://github.com/japaric/xargo).

    cargo install xargo
    

## Setup
### Environment
Paths are specific to my setup and locations. This can vary in your case and you
have to adjust them.

Add your cross-compiler to the search path

    set PATH=C:\Toolchain\MIPS\mips-2016.05\bin;%PATH%

Specify the compiler setup to use:

    set CC=mips-linux-gnu-gcc
    set CFLAGS_mipsel-unknown-linux-gnu=-EL
    set AR_mipsel-unknown-linux-gnu=mips-linux-gnu-ar

* where *CC* specifies the compiler you use to build the target and std crate binaries
* *CFLAGS_mipsel-unknown-linux-gnu* the specific compiler switches to use. In this case
  *-EL* tells the compiler to build code for little endian processors (the el in mipsel).
  Other compilers may come for a specific architecture and are named `mipsel-linux-gnu-gcc`,
  but in my case the compiler is `mips-linux-gnu-gcc` and can output in both endianess. You
  have to consider this when using a different tool chain.
  To learn more about the capabilities of the compiler use `mips-linux-gnu-gcc --target-help`.
* The *AR_mipsel-unknown-linux-gnu* binary util is used to create, modify, and extract from 
  archives (libraries). 

### Build files

Any minimal `Cargo.toml`. Nothing special:

    [package]
    name = "hello-vuduo"
    version = "0.1.0"
    authors = ["Jedzia <jed69@gmx.de>"]
    edition = "2018"


Build file for Xargo `Xargo.toml`:

    [dependencies]
    std = {features = ["panic_unwind"]}
    
    [build]
    target = "mipsel-linux-gnu-gcc"

* **std**: The std dependency is what you want to build on your own.
  This is the cause of `Illegal instruction` errors on mips32**r1** targets as the
  upstream target from the Rust repository is built for mips32**r2** with fpxx and
  nooddspreg enabled. This change was done on Mar 15, 2018:
  [bump mipsel isa leval and enable fpxx #48874](https://github.com/rust-lang/rust/pull/48874)
* The **[build] target** specifies the default architecture to build. 
  Can also be specified at the command-line with
    
        cargo build --target=mipsel-unknown-linux-gnu
        xargo build --target=my-unknown-system-dos

### Cargo configuration

Local [cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html) `.cargo/config`:

    [build]
    target = "mipsel-unknown-linux-gnu"
    
    [target.mipsel-unknown-linux-gnu]
    rustflags = [
      "-C", "link-arg=-EL",
      "-C", "target-cpu=mips32",
      "-C", "target-feature=+mips32,-mips32r2,-fpxx,-nooddspreg",
    ]
    
    ar = "mips-linux-gnu-ar"
    linker = "mips-linux-gnu-gcc"
    runner = "run.bat"  

* The **[build] target** specifies the default architecture to build (see above).
* Under **[target.mipsel-unknown-linux-gnu]** we specify the flags used for creating the binary
  and linking it. **-C** is forwarded to the rustc-compiler and indicates a code-generation option.
  `rustc -C help` will list all options available. The default target specs for the current rustc 
  compiler can be listed with: 
  `rustc -Z unstable-options --print target-spec-json --target mipsel-unknown-linux-gnu`.
  The used options in detail:
  - link-arg=-EL is daisy-chained to the linker(mips-linux-gnu-gcc): little endian output is generated.
  - target-cpu=mips32 for use with our good old mips32r1. A list can be retrieved with
    `rustc --print target-cpus`. We are building for mips so with our target: 
    `rustc --target mipsel-unknown-linux-gnu --print target-cpus`.
  - target-feature: For a list of available options use `rustc --print target-features` or in our case:
    `rustc --target mipsel-unknown-linux-gnu -C target-cpu=mips32 -C target-feature=help`. 
    * +mips32: Enable Mips32 ISA Support.
    * -mips32r2: Disable Mips32r2 ISA Support ... illegal instruction.
    * -fpxx: Disable support for FPXX. The FPXX extension mandates that all code must execute correctly
      when run   using 32-bit or 64-bit registers. The code can be interlinked with either FP32 or FP64,
      but not both.
    * -nooddspreg: Disable odd numbered single-precision registers.
* ar: binary util used to create, modify, and extract from archives (libraries). Use our target
  specific one.
* linker: mips-linux-gnu-gcc is used as linker in conjunction with the specified 
  link-arg=-EL flag to produce little endian binaries.
* runner: What (x/c)argo uses to execute the **run** command. The first command-line parameter is
  the relative path of your binary. I use a combination of scp to the target-box or copy to a network 
  share and ssh to run it remotely. 
  Something like:
        run.bat:
        cp %1 \\VUDUO2/Harddisk/jedzia/rust
        ssh jedzia@vuduo2 /media/hdd/jedzia/rust/hello-vuduo 
  
> I was able to successfully compile and run with the fpxx and nooddspreg options enabled 
> (`"-C", "target-feature=+mips32,-mips32r2,+fpxx,+nooddspreg"`). 
> Maybe they are not necessary for
> the VU+ Duo2. But i can't prove this as i only compile a little hello world with floating point
> arithmetic in it. When it fails you have to tweak it for your target. I have disabled these options
> as they are part of the #48874 change set (see above).

## Compiling

run

    xargo build

or if you have set up a runner in `.cargo/config` for remote execution
 
    xargo run 
to see it work.

and 

    cargo build
to see it fail again with illegal instruction as it is compiled against the Rust 
*mipsel-unknown-linux-gnu* target and not our custom one built by xargo.