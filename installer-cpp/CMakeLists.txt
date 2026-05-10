cmake_minimum_required(VERSION 3.10)
project(linux-sys-monitor-installer)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_RC_COMPILER_INIT x86_64-w64-mingw32-windres)

include_directories(${CMAKE_SOURCE_DIR}/src)

enable_language(C ASM RC)

set(RC_COMPILE "x86_64-w64-mingw32-windres")
set(CMAKE_RC_COMPILE_OBJECT
    "<CMAKE_RC_COMPILER> <DEFINES> <INCLUDES> <FLAGS> -o <OBJECT> <SOURCE>")

set(SOURCES
    src/main.cpp
    src/window.cpp
    src/install.cpp
)

add_executable(installer WIN32 ${SOURCES})

set_target_properties(installer PROPERTIES
    OUTPUT_NAME "installer"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_SOURCE_DIR}/release"
)

target_link_libraries(installer
    -lkernel32
    -luser32
    -lshell32
    -lshfolder
    -lcomdlg32
    -lcomctl32
    -lole32
    -loleaut32
    -luuid
    -lwininet
    -lmpr
)

set_target_properties(installer PROPERTIES
    LINK_FLAGS "/SUBSYSTEM:WINDOWS /ENTRY:wWinMainCRTStartup"
)

if(WIN32)
    add_custom_command(TARGET installer POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        $<TARGET_FILE:installer>
        "${CMAKE_SOURCE_DIR}/release/installer.exe"
    )
endif()
