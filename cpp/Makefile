# This is the default target, which will be built when 
# you invoke make
# or "cs-make install" to put on Vu+ DUO2
.PHONY: all

#CC?=mips-linux-gnu-g++
CXX=mips-linux-gnu-g++
CXXFLAGS=-EL -mips32

TARGETDIR=\\VUDUO2X/Harddisk/jedzia/rust

SCP="C:/Program Files/Git/usr/bin/scp.exe"
SSH="C:/Program Files/Git/usr/bin/ssh.exe" 

all: hello

# This rule tells make how to build hello from hello.cpp
hello: hello.cpp
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -o hello hello.cpp

# This rule tells make to copy hello to the binaries subdirectory,
# creating it if necessary
.PHONY: install
install: all
#	mkdir -p "$(TARGETDIR)"
#	cp -p hello "$(TARGETDIR)/"
	cp hello "$(TARGETDIR)/"
#	copy -p hello "$(TARGETDIR)/"

run: install
	$(SSH) root@vuduo2x /media/hdd/jedzia/rust/hello

# This rule tells make to delete hello and hello.o
.PHONY: clean 
clean:
	rm -f hello