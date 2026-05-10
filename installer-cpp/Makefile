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
OBJECTS = $(OBJDIR)/main.o $(OBJDIR)/window.o $(OBJDIR)/install.o

MANIFEST = $(OBJDIR)/app.manifest

TARGET = $(RELEASEDIR)/installer.exe

.PHONY: all clean dirs

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

$(OBJDIR)/app.manifest:
	@echo '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>' > $@
	@echo '<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">' >> $@
	@echo '  <assemblyIdentity version="1.0.0.0" processorArchitecture="*" name="LinuxSysMonitorInstaller" type="win32"/>' >> $@
	@echo '  <description>Linux System Monitor Installer</description>' >> $@
	@echo '  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">' >> $@
	@echo '    <security>' >> $@
	@echo '      <requestedPrivileges>' >> $@
	@echo '        <requestedExecutionLevel level="asInvoker" uiAccess="false"/>' >> $@
	@echo '      </requestedPrivileges>' >> $@
	@echo '    </security>' >> $@
	@echo '  </trustInfo>' >> $@
	@echo '  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">' >> $@
	@echo '    <application>' >> $@
	@echo '      <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>' >> $@
	@echo '      <supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}"/>' >> $@
	@echo '      <supportedOS Id="{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}"/>' >> $@
	@echo '      <supportedOS Id="{35138b9a-5d96-4fbd-8e2d-a2440225f93a}"/>' >> $@
	@echo '    </application>' >> $@
	@echo '  </compatibility>' >> $@
	@echo '  <dependency>' >> $@
	@echo '    <dependentAssembly>' >> $@
	@echo '      <assemblyIdentity' >> $@
	@echo '        type="win32"' >> $@
	@echo '        name="Microsoft.Windows.Common-Controls"' >> $@
	@echo '        version="6.0.0.0"' >> $@
	@echo '        processorArchitecture="*"' >> $@
	@echo '        publicKeyToken="6595b64144ccf1df"' >> $@
	@echo '        language="*"' >> $@
	@echo '        />' >> $@
	@echo '    </dependentAssembly>' >> $@
	@echo '  </dependency>' >> $@
	@echo '</assembly>' >> $@

$(RELEASEDIR)/installer.exe: $(OBJECTS) $(MANIFEST)
	$(CC) $(CXXFLAGS) $(OBJECTS) $(LDFLAGS) -mwindows -o $@
	$(STRIP) $@

clean:
	rm -rf $(OBJDIR) $(RELEASEDIR)

rebuild: clean all

check:
	@echo "=== Checking toolchain ==="
	@which $(CC) && $(CC) --version | head -1 || echo "Error: $(CC) not found"
	@which $(WINDRES) && $(WINDRES) --version | head -1 || echo "Error: $(WINDRES) not found"
	@echo "=== Target Windows version: 0x$(WINVER) (Windows 7) ==="
