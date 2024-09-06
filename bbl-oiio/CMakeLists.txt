cmake_minimum_required(VERSION 3.15)
project(oiio VERSION 0.1 LANGUAGES C CXX)

find_package(OpenImageIO 2.5 REQUIRED)
find_package(babble 0.8 CONFIG REQUIRED)

include(GNUInstallDirs)

# The bindfile contains all our binding definitions
set(bindfiles
  bind/bbl-misc.cpp
  bind/attrdelegate.cpp
  bind/deepdata.cpp
  bind/filesystem.cpp
  bind/filter.cpp
  bind/imagebuf.cpp
  bind/imagebufalgo.cpp
  bind/imagecache.cpp
  bind/imageio.cpp
  bind/paramlist.cpp
  bind/texture.cpp
  bind/typedesc.cpp
  bind/ustring.cpp
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

# This is just here to trigger generation of compile commands for the bind file so we get LSP functionality in the bind file
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
