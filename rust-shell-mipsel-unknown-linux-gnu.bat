@echo off
REM set PATH=C:\Toolchain\MIPS\gcc-4.9.2-mips-unknown-linux-gnu\bin;C:\Toolchain\MIPS\gcc-4.9.2-mips-unknown-linux-gnu\libexec\gcc\mips-unknown-linux-gnu\4.9.2;%PATH%
set PATH=C:\Toolchain\MIPS\mips-2016.05\bin;%PATH%

set CC=mips-linux-gnu-gcc
set CFLAGS_mipsel-unknown-linux-gnu=-EL
set AR_mipsel-unknown-linux-gnu=mips-linux-gnu-ar

REM call C:\D\ldc2vars64.bat
rem e:
rem cd E:\Projects\D

echo .
echo .
echo cargo build --target=mipsel-unknown-linux-gnu
echo xargo build --target=mipsel-unknown-linux-gnu
echo .
cmd