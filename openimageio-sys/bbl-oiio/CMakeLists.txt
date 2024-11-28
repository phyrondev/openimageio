cmake_minimum_required(VERSION 3.15)
project(oiio VERSION 0.1 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER "clang++")
set(CMAKE_C_COMPILER "clang")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} \
-isystem/usr/lib/llvm-19/lib/clang/19/include \
--no-system-header-prefix=/usr/lib/gcc/x86_64-linux-gnu/14/../../../../include/c++/14 \
--no-system-header-prefix=/usr/lib/gcc/x86_64-linux-gnu/14/../../../../include/x86_64-linux-gnu/c++/14 \
--no-system-header-prefix=/usr/lib/gcc/x86_64-linux-gnu/14/../../../../include/c++/14/backward \
")

find_package(
    OpenImageIO 3
    REQUIRED
    HINTS
    ${OIIO_DIST}
)
find_package(babble 0.8 CONFIG REQUIRED)

include(GNUInstallDirs)

# The bindfile contains all our binding definitions.
set(bindfiles
    bind/attrdelegate.cpp
    bind/bbl-misc.cpp
    bind/color.cpp
    bind/deepdata.cpp
    bind/filesystem.cpp
    bind/filter.cpp
    bind/imagebuf.cpp
    bind/imagebufalgo.cpp
    bind/imagecache.cpp
    bind/imageio.cpp
    bind/imagespec.cpp
    bind/paramlist.cpp
    bind/texture.cpp
    bind/typedesc.cpp
    bind/ustring.cpp
    bind/vecparam.cpp
)

bbl_translate_binding(
    oiio
    BINDFILES
        ${bindfiles}
    COMPILE_ARGS
        -DNOMINMAX
        -D_MT
        -Wno-macro-redefined
)

target_link_libraries(oiio-c PUBLIC OpenImageIO::OpenImageIO)
target_compile_definitions(oiio-c PRIVATE NOMINMAX)

set_target_properties(oiio-c PROPERTIES CXX_STANDARD 17)

# This is just here to trigger generation of compile commands for the bind file
# so we get LSP functionality in the bind file.
add_library(bind-dummy ${bindfiles})
target_link_libraries(bind-dummy babble::bind OpenImageIO::OpenImageIO)

install(
    TARGETS
        oiio-c
    LIBRARY
        DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE
        DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME
        DESTINATION ${CMAKE_INSTALL_BINDIR}
    INCLUDES
        DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)
