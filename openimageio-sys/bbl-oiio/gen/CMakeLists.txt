cmake_minimum_required(VERSION 3.15)
project(bbl-gen-oiio VERSION 0.1 LANGUAGES C CXX)

set(CMAKE_CXX_COMPILER "clang++")
set(CMAKE_C_COMPILER "clang")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} \
-isystem/usr/lib/llvm-19/lib/clang/19/include / --no-system-header-prefix=/usr/lib/gcc/x86_64-linux-gnu/14/../../../../include/c++/14 \
--no-system-header-prefix=/usr/lib/gcc/x86_64-linux-gnu/14/../../../../include/x86_64-linux-gnu/c++/14 \
--no-system-header-prefix=/usr/lib/gcc/x86_64-linux-gnu/14/../../../../include/c++/14/backward \
")

find_package(babble 0.8 CONFIG REQUIRED)
find_package(OpenImageIO 3 REQUIRED)

include(GNUInstallDirs)

bbl_generate_binding(
    oiio
    gen.cpp
    OpenImageIO_v3
)

target_link_libraries(oiio-gen PUBLIC OpenImageIO::OpenImageIO)
target_compile_definitions(oiio-gen PRIVATE NOMINMAX BOOST_ALL_NO_LIB __TBB_show_deprecation_message_task_H)
