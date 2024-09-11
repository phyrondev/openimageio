#include <babble>

#include <OpenImageIO/color.h>

BBL_MODULE(oiio) {

  bbl::Class<OIIO::ColorConfig>().ctor(bbl::Class<OIIO::ColorConfig>::Ctor<>(),
                                       "default");
}
