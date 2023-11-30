#include <babble>

#include <OpenImageIO/imagebuf.h>

BBL_MODULE(oiio) {
    bbl::fn(&OIIO::openimageio_version);

    bbl::fn(&OIIO::has_error);

    bbl::fn(&OIIO::geterror);

    // bbl::fn((bool (*)(string_view, TypeDesc, const void *))
    //         &OIIO::attribute, "attribute_00");

    // bbl::fn((bool (*)(string_view, TypeDesc, void *))
    //         &OIIO::getattribute, "getattribute_00");
}
