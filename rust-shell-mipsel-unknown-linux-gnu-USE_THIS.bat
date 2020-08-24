@echo off
REM set PATH=C:\Toolchain\MIPS\gcc-4.9.2-mips-unknown-linux-gnu\bin;C:\Toolchain\MIPS\gcc-4.9.2-mips-unknown-linux-gnu\libexec\gcc\mips-unknown-linux-gnu\4.9.2;%PATH%
set PATH=C:\Toolchain\MIPS\mips-2016.05\bin;C:\Toolchain\mingw-w64\x86_64-6.2.0-posix-seh-rt_v5-rev1\mingw64\bin;C:\msys64\usr\bin;%PATH%

set CC=mips-linux-gnu-gcc
set CFLAGS_mipsel-unknown-linux-gnu=-EL -mips32 -msoft-float
set AR_mipsel-unknown-linux-gnu=mips-linux-gnu-ar

set OPENSSL_DIR=C:\Toolchain\MIPS\openssl\usr

set MAKEFLAGS=-j8

REM call C:\D\ldc2vars64.bat
rem e:
rem cd E:\Projects\D

echo .
echo .
echo xargo -Z build-std build --target=mipsel-unknown-linux-gnu
echo xargo -Z build-std run --target=mipsel-unknown-linux-gnu -- -o options
echo or since Rust nightly 2020-07-15
echo cargo -Z build-std build --target=mipsel-unknown-linux-gnu
echo cargo -Z build-std run --target=mipsel-unknown-linux-gnu -- -o options
echo or local on the host:
echo cargo build --target=x86_64-pc-windows-gnu
echo .
cmd