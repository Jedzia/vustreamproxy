call C:\D\dmd2vars64ldc2.bat

set PATH=C:\Toolchain\MIPS\mips-2016.05\bin;%PATH%

set CC=mips-linux-gnu-gcc
set CFLAGS_mipsel-unknown-linux-gnu=-EL
set AR_mipsel-unknown-linux-gnu=mips-linux-gnu-ar

start "" "C:\Program Files\JetBrains\CLion 2018.3.2\bin\clion64.exe"
