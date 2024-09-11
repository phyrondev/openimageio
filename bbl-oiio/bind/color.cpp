#include <babble>

#include <OpenImageIO/color.h>

BBL_MODULE(oiio) {

  bbl::Class<OIIO::ColorConfig>()
      .ctor(bbl::Class<OIIO::ColorConfig>::Ctor<>(), "default")
      .ctor(bbl::Class<OIIO::ColorConfig>::Ctor<OIIO::string_view>("file_name"),
            "ctor")
      .m(&OIIO::ColorConfig::has_error)
      .m(&OIIO::ColorConfig::geterror);
}
