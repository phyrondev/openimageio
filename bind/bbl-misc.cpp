#include <babble>

#include <OpenImageIO/imagebuf.h>
#include <string>

BBL_MODULE(oiio) {
    bbl::fn(&OIIO::openimageio_version);

    bbl::fn(&OIIO::has_error);

    bbl::fn(&OIIO::geterror);

    bbl::Class<std::string>("String")
        .m(&std::string::c_str)
    ;

    // bbl::Class<std::wstring>("WString")
    //     .m(&std::wstring::c_str)
    // ;

    // bbl::fn((bool (*)(string_view, TypeDesc, const void *))
    //         &OIIO::attribute, "attribute_00");

    // bbl::fn((bool (*)(string_view, TypeDesc, void *))
    //         &OIIO::getattribute, "getattribute_00");
}
