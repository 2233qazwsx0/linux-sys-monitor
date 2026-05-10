CC = x86_64-w64-mingw32-g++
WINDRES = x86_64-w64-mingw32-windres
STRIP = x86_64-w64-mingw32-strip

CXXFLAGS = -std=c++11 -O2 -Wall -Wextra
CFLAGS = -O2 -Wall

LDFLAGS = -lkernel32 -luser32 -lshell32 -lshfolder -lcomdlg32 -lcomctl32 \
          -lole32 -loleaut32 -luuid -lwininet -lmpr -static-libgcc -static-libstdc++

WINVER = 0x0601
WINVERFLAGS = -D_WIN32_WINNT=$(WINVER) -DWINVER=$(WINVER)

INCLUDES = -I./src

SRCDIR = src
RELEASEDIR = release
OBJDIR = obj

SOURCES = $(SRCDIR)/main.cpp $(SRCDIR)/window.cpp $(SRCDIR)/install.cpp
OBJECTS = $(OBJDIR)/main.o $(OBJDIR)/window.o $(OBJDIR)/install.o $(OBJDIR)/app.res

TARGET = $(RELEASEDIR)/installer.exe

.PHONY: all clean dirs i686 x86_64

all: dirs $(TARGET)

dirs:
	@mkdir -p $(OBJDIR)
	@mkdir -p $(RELEASEDIR)

$(OBJDIR)/main.o: $(SRCDIR)/main.cpp $(SRCDIR)/resource.h
	$(CC) $(CXXFLAGS) $(WINVERFLAGS) $(INCLUDES) -c $< -o $@

$(OBJDIR)/window.o: $(SRCDIR)/window.cpp $(SRCDIR)/window.h $(SRCDIR)/resource.h
	$(CC) $(CXXFLAGS) $(WINVERFLAGS) $(INCLUDES) -c $< -o $@

$(OBJDIR)/install.o: $(SRCDIR)/install.cpp $(SRCDIR)/install.h
	$(CC) $(CXXFLAGS) $(WINVERFLAGS) $(INCLUDES) -c $< -o $@

$(OBJDIR)/app.res: $(SRCDIR)/app.rc $(SRCDIR)/app.manifest
	$(WINDRES) -I$(SRCDIR) $< -O coff -o $@

$(RELEASEDIR)/installer.exe: $(OBJECTS)
	$(CC) $(CXXFLAGS) $(OBJECTS) $(LDFLAGS) -mwindows -o $@
	$(STRIP) $@

i686:
	@mkdir -p $(OBJDIR)-i686 $(RELEASEDIR)-i686
	@make -e CC=i686-w64-mingw32-g++ WINDRES=i686-w64-mingw32-windres STRIP=i686-w64-mingw32-strip OBJDIR=$(OBJDIR)-i686 RELEASEDIR=$(RELEASEDIR)-i686 TARGET=$(RELEASEDIR)-i686/installer.exe clean all

x86_64:
	@make clean all

clean:
	rm -rf $(OBJDIR) $(OBJDIR)-i686 $(RELEASEDIR) $(RELEASEDIR)-i686

rebuild: clean all

check:
	@echo "=== Checking toolchain ==="
	@which $(CC) && $(CC) --version | head -1 || echo "Error: $(CC) not found"
	@which $(WINDRES) && $(WINDRES) --version | head -1 || echo "Error: $(WINDRES) not found"
	@echo "=== Target Windows version: 0x$(WINVER) (Windows 7) ==="
