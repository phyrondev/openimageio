#include <babble>

#include <OpenImageIO/imagebufalgo.h>

BBL_MODULE(oiio) {

    bbl::Class<OIIO::ImageBufAlgo::CompareResults>()
        .value_type()
        .f(&OIIO::ImageBufAlgo::CompareResults::meanerror)
        .f(&OIIO::ImageBufAlgo::CompareResults::rms_error)
        .f(&OIIO::ImageBufAlgo::CompareResults::PSNR)
        .f(&OIIO::ImageBufAlgo::CompareResults::maxerror)
        .f(&OIIO::ImageBufAlgo::CompareResults::maxx)
        .f(&OIIO::ImageBufAlgo::CompareResults::maxy)
        .f(&OIIO::ImageBufAlgo::CompareResults::maxz)
        .f(&OIIO::ImageBufAlgo::CompareResults::maxc)
        .f(&OIIO::ImageBufAlgo::CompareResults::nwarn)
        .f(&OIIO::ImageBufAlgo::CompareResults::nfail)
        .f(&OIIO::ImageBufAlgo::CompareResults::error);

    // zero()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::zero, "ImageBufAlgo_zero");

    // fill()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::cspan<float>, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::fill, "ImageBufAlgo_fill");

    // fill_vertical()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::cspan<float>, OIIO::cspan<float>, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::fill, "ImageBufAlgo_fill_vertical");

    // fill_corners()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::cspan<float>, OIIO::cspan<float>, OIIO::cspan<float>, OIIO::cspan<float>, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::fill, "ImageBufAlgo_fill_corners");

    // noise()
    bbl::fn((bool (*)(OIIO::ImageBuf&, OIIO::string_view, float, float, bool, int, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::noise, "ImageBufAlgo_noise");

    // over()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::over, "ImageBufAlgo_over");

    // from_over()
    bbl::fn((OIIO::ImageBuf (*)(const OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::over, "ImageBufAlgo_from_over");

    // zover()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, const OIIO::ImageBuf&, bool, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::zover, "ImageBufAlgo_zover");

    // rotate()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, float, OIIO::string_view, float, bool, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::rotate, "ImageBufAlgo_rotate");

    // rotate_()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, float, float, float,  OIIO::string_view, float, bool, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::rotate, "ImageBufAlgo_rotate_around");

    // rotate90()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::rotate90, "ImageBufAlgo_rotate90");

    // rotate180()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::rotate180, "ImageBufAlgo_rotate180");

    // rotate270()
    bbl::fn((bool (*)(OIIO::ImageBuf&, const OIIO::ImageBuf&, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::rotate270, "ImageBufAlgo_rotate270");

    bbl::fn((OIIO::ImageBufAlgo::CompareResults (*)(const OIIO::ImageBuf&, const OIIO::ImageBuf&, float, float, OIIO::ROI, int))
        &OIIO::ImageBufAlgo::compare, "ImageBufAlgo_compare");

}
