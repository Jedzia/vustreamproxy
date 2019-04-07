call C:\D\dmd2vars64ldc2.bat

set PATH=C:\Toolchain\MIPS\mips-2016.05\bin;C:\Toolchain\mingw-w64\x86_64-6.2.0-posix-seh-rt_v5-rev1\mingw64\bin;%PATH%

set CC=mips-linux-gnu-gcc
set CFLAGS_mipsel-unknown-linux-gnu=-EL -mips32
set AR_mipsel-unknown-linux-gnu=mips-linux-gnu-ar

set MAKEFLAGS=-j8

start "" "C:\Program Files\JetBrains\CLion 2018.3.2\bin\clion64.exe"
