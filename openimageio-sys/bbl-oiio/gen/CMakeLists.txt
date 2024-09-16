cmake_minimum_required(VERSION 3.15)
project(bbl-gen-oiio VERSION 0.1 LANGUAGES C CXX)

find_package(babble 0.5 CONFIG REQUIRED)
find_package(OpenImageIO 2.3 REQUIRED)

include(GNUInstallDirs)

bbl_generate_binding(
    oiio
    gen.cpp
    OpenImageIO_v2_3
)

target_link_libraries(oiio-gen PUBLIC OpenImageIO::OpenImageIO)
target_compile_definitions(oiio-gen PRIVATE NOMINMAX BOOST_ALL_NO_LIB __TBB_show_deprecation_message_task_H)