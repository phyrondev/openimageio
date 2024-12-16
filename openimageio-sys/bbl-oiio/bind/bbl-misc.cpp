#include <babble>

#include <Imath/half.h>
#include <OpenImageIO/half.h>
#include <OpenImageIO/imagebuf.h>
#include <OpenImageIO/paramlist.h>
#include <OpenImageIO/span.h>
#include <OpenImageIO/strongparam.h>

#include <string>

BBL_MODULE(oiio) {

  bbl::fn(&OIIO::openimageio_version);

  bbl::fn(&OIIO::has_error);

  bbl::fn(&OIIO::geterror);

  bbl::Class<Imath::half>("Half");

  /*bbl::Class<OIIO::StrongParam<Bool, bool>>("StrongParamBool")
      .ctor(bbl::Class<OIIO::StrongParam<Bool, bool>>::Ctor<bool>("value"),
      "ctor");*/

  bbl::Class<OIIO::ParamValueSpan>("ParamValueSpan")
      .ctor(
          bbl::Class<OIIO::ParamValueSpan>::Ctor<const OIIO::ParamValueList &>(
              "list"),
          "ctor");

  bbl::Class<OIIO::cspan<float>>("CspanF32")
      .ctor(
          bbl::Class<OIIO::cspan<float>>::Ctor<float *, size_t>("data", "size"),
          "ctor");

  bbl::Class<OIIO::cspan<half>>("CspanF16")
      .ctor(bbl::Class<OIIO::cspan<half>>::Ctor<half *, size_t>("data", "size"),
            "ctor");

  bbl::Class<OIIO::cspan<double>>("CspanF64")
      .ctor(bbl::Class<OIIO::cspan<double>>::Ctor<double *, size_t>("data",
                                                                    "size"),
            "ctor");

  bbl::Class<OIIO::cspan<u_int64_t>>("CspanU63")
      .ctor(bbl::Class<OIIO::cspan<u_int64_t>>::Ctor<u_int64_t *, size_t>(
                "data", "size"),
            "ctor");

  bbl::Class<OIIO::cspan<u_int32_t>>("CspanU32")
      .ctor(bbl::Class<OIIO::cspan<u_int32_t>>::Ctor<u_int32_t *, size_t>(
                "data", "size"),
            "ctor");

  bbl::Class<OIIO::cspan<int32_t>>("CspanI32")
      .ctor(bbl::Class<OIIO::cspan<int32_t>>::Ctor<int32_t *, size_t>("data",
                                                                      "size"),
            "ctor");

  bbl::Class<OIIO::cspan<u_int16_t>>("CspanU16")
      .ctor(bbl::Class<OIIO::cspan<u_int16_t>>::Ctor<u_int16_t *, size_t>(
                "data", "size"),
            "ctor");

  bbl::Class<OIIO::cspan<u_int8_t>>("CspanU8").ctor(
      bbl::Class<OIIO::cspan<u_int8_t>>::Ctor<u_int8_t *, size_t>("data",
                                                                  "size"),
      "ctor");

  bbl::Class<OIIO::cspan<std::string>>("CspanString")
      .ctor(bbl::Class<OIIO::cspan<std::string>>::Ctor<std::string *, size_t>(
                "data", "size"),
            "ctor");

  bbl::Class<std::string>("String")
      .ctor(bbl::Class<std::string>::Ctor(), "ctor_default")
      .ctor(bbl::Class<std::string>::Ctor<const char *, size_t>("s", "count"),
            "ctor")
      .m(&std::string::c_str)
      .m((const char *(std::string::*)() const) & std::string::data, "data")
      .m(&std::string::empty)
      .m(&std::string::size)
      .m(&std::string::length);

  bbl::Class<OIIO::string_view>("StringView")
      .ctor(bbl::Class<OIIO::string_view>::Ctor(), "default")
      .ctor(bbl::Class<OIIO::string_view>::Ctor<const char *, size_t>("s",
                                                                      "count"),
            "ctor")
      .m(&OIIO::string_view::data)
      .m(&OIIO::string_view::size)
      .m(&OIIO::string_view::empty)
      .m(&OIIO::string_view::length);

  // bbl::Class<std::wstring>("WString")
  //     .m(&std::wstring::c_str)
  // ;

  // bbl::fn((bool (*)(string_view, TypeDesc, const void *))
  //         &OIIO::attribute, "attribute_00");

  // bbl::fn((bool (*)(string_view, TypeDesc, void *))
  //         &OIIO::getattribute, "getattribute_00");
}
