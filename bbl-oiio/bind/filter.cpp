#include <babble>

#include <OpenImageIO/filter.h>


BBL_MODULE(oiio) {

    bbl::Class<OIIO::Filter2D>()
        .m((OIIO::Filter2D* (OIIO::Filter2D::*)(OIIO::string_view, float, float))
            &OIIO::Filter2D::create, "create")
        .m(&OIIO::Filter2D::destroy)
}