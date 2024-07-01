#include <babble>

#include <OpenImageIO/imagebufalgo.h>

BBL_MODULE(oiio) {

    // zero()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::zero, "ImageBufAlgo_zero");

    // fill()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::cspan<float>, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::fill, "ImageBufAlgo_fill_00");

    // over()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::over, "ImageBufAlgo_over_self");

    // over()
    bbl::fn((OIIO::ImageBuf (*)(const OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::over, "ImageBufAlgo_over");

    // zover()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, const OIIO::ImageBuf&, bool, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::zover, "ImageBufAlgo_zover_self");
}
