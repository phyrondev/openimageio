#include <babble>

#include <OpenImageIO/filesystem.h>

BBL_MODULE(oiio) {

  bbl::Enum<OIIO::Filesystem::IOProxy::Mode>();

  // bbl::ClassIncomplete<FILE>();

  bbl::Class<OIIO::Filesystem::IOFile>().ctor(
      bbl::Class<OIIO::Filesystem::IOFile>::Ctor<
          OIIO::string_view, OIIO::Filesystem::IOProxy::Mode>("file_name",
                                                              "mode"),
      "ctor");
  //.ctor(bbl::Class<OIIO::Filesystem::IOFile>::Ctor<FILE*,
  //OIIO::Filesystem::IOProxy::Mode>("file", "mode"), "new");

  /*bbl::Class<OIIO::Filesystem::IOFile>()
      .ctor(bbl::Class<OIIO::Filesystem::IOFile>::Ctor(), "default" )
      .ctor(bbl::Class<OIIO::Filesystem::IOFile>::Ctor<OIIO::string_view,
     OIIO::Filesystem::IOProxy::Mode>("file_name", "mode"), "ctor");
      */
}
