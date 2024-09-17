#include <babble>

// #include <OpenImageIO/Imath.h>
#include <OpenImageIO/vecparam.h>

BBL_MODULE(oiio) {

  // bbl::Class<Imath::M33f>().value_type().f(&Imath::M33f::x);

  bbl::Class<OIIO::M33fParam>("oiio_M33fParam_t")
      //.ctor(bbl::Class<OIIO::M33fParam>::Ctor<const float[9] &>("data"),
      //"ctor")
      .m(&OIIO::M33fParam::data);
}
