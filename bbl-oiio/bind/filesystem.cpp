#include <babble>

#include <OpenImageIO/filesystem.h>

BBL_MODULE(oiio) {

    bbl::Enum<OIIO::Filesystem::IOProxy::Mode>();

    bbl::Class<OIIO::Filesystem::IOProxy>()
        .ctor(bbl::Class<OIIO::Filesystem::IOProxy>::Ctor<OIIO::string_view, OIIO::Filesystem::IOProxy::Mode>("file_name", "mode"), "ctor");

}
