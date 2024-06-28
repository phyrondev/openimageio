#include <babble>

#include <OpenImageIO/imagebuf.h>
#include <OpenImageIO/span.h>

#include <string>

BBL_MODULE(oiio) {
    bbl::fn(&OIIO::openimageio_version);

    bbl::fn(&OIIO::has_error);

    bbl::fn(&OIIO::geterror);

    bbl::Class<OIIO::cspan<float>>("CspanF32")
        .ctor(bbl::Class<OIIO::cspan<float>>::Ctor<float*, size_t>("data", "size"), "ctor" )
    ;

    bbl::Class<std::string>("String")
        .m(&std::string::c_str)
        .m((const char* (std::string::*)() const)
           &std::string::data, "data")
        .m(&std::string::size)
    ;

    bbl::Class<OIIO::string_view>("StringView")
        .ctor(bbl::Class<OIIO::string_view>::Ctor<const char*, size_t>("s", "count"), "ctor" )
        .m(&OIIO::string_view::data)
        .m(&OIIO::string_view::size)

    ;

    // bbl::Class<std::wstring>("WString")
    //     .m(&std::wstring::c_str)
    // ;

    // bbl::fn((bool (*)(string_view, TypeDesc, const void *))
    //         &OIIO::attribute, "attribute_00");

    // bbl::fn((bool (*)(string_view, TypeDesc, void *))
    //         &OIIO::getattribute, "getattribute_00");
}
