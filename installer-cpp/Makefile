CC = x86_64-w64-mingw32-g++
WINDRES = x86_64-w64-mingw32-windres
STRIP = x86_64-w64-mingw32-strip

CXXFLAGS = -std=c++11 -O2 -Wall -Wextra -D_WIN32_WINNT=0x0601 -DWINVER=0x0601
LDFLAGS = -lkernel32 -luser32 -lshell32 -lcomdlg32 -lcomctl32 -lole32 -loleaut32 -lwininet

SRC = src/main.cpp src/window.cpp src/install.cpp
OBJ = obj/main.o obj/window.o obj/install.o obj/app.res

all: dirs release/installer.exe

dirs:
	@mkdir -p obj release

obj/main.o: src/main.cpp src/resource.h
	$(CC) $(CXXFLAGS) -I./src -c $< -o $@

obj/window.o: src/window.cpp src/window.h src/resource.h
	$(CC) $(CXXFLAGS) -I./src -c $< -o $@

obj/install.o: src/install.cpp src/install.h
	$(CC) $(CXXFLAGS) -I./src -c $< -o $@

obj/app.res: src/app.rc src/app.manifest
	$(WINDRES) -I./src $< -O coff -o $@

release/installer.exe: $(OBJ)
	$(CC) $(OBJ) $(LDFLAGS) -mwindows -o $@
	$(STRIP) $@

clean:
	rm -rf obj release obj-i686 release-i686

.PHONY: all dirs clean
