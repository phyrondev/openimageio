#include <babble>

#include <OpenImageIO/imagebufalgo.h>

BBL_MODULE(oiio) {

    // zero()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::zero, "ImageBufAlgo_zero");

    // fill()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::cspan<float>, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::fill, "ImageBufAlgo_fill_00");

    // noise()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::string_view, float, float, bool, int, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::noise, "ImageBufAlgo_noise");

    // over()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::over, "ImageBufAlgo_over");

    // over()
    bbl::fn((OIIO::ImageBuf (*)(const OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::over, "ImageBufAlgo_from_over");

    // zover()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, const OIIO::ImageBuf&, bool, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::zover, "ImageBufAlgo_zover");
}
