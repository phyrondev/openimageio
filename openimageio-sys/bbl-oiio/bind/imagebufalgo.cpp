#include <babble>

#include <OpenImageIO/imagebufalgo.h>
#include <OpenImageIO/paramlist.h>

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

  bbl::Enum<OIIO::ImageBufAlgo::TextAlignX>();
  bbl::Enum<OIIO::ImageBufAlgo::TextAlignY>();

  // zero()
  bbl::fn((bool (*)(OIIO::ImageBuf &, OIIO::ROI, int))&OIIO::ImageBufAlgo::zero,
          "ImageBufAlgo_zero");

  // checker()
  bbl::fn((bool (*)(OIIO::ImageBuf &, int, int, int, OIIO::cspan<float>,
                    OIIO::cspan<float>, int, int, int, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::checker,
          "ImageBufAlgo_checker");

  // fill()
  bbl::fn((bool (*)(OIIO::ImageBuf &, OIIO::cspan<float>, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::fill,
          "ImageBufAlgo_fill");

  // fill_vertical()
  bbl::fn((bool (*)(OIIO::ImageBuf &, OIIO::cspan<float>, OIIO::cspan<float>,
                    OIIO::ROI, int))&OIIO::ImageBufAlgo::fill,
          "ImageBufAlgo_fill_vertical");

  // fill_corners()
  bbl::fn((bool (*)(OIIO::ImageBuf &, OIIO::cspan<float>, OIIO::cspan<float>,
                    OIIO::cspan<float>, OIIO::cspan<float>, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::fill,
          "ImageBufAlgo_fill_corners");

  // noise()
  bbl::fn((bool (*)(OIIO::ImageBuf &, OIIO::string_view, float, float, bool,
                    int, OIIO::ROI, int))&OIIO::ImageBufAlgo::noise,
          "ImageBufAlgo_noise");

  // over()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &,
                    const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::over,
          "ImageBufAlgo_over");

  // from_over()
  bbl::fn((OIIO::ImageBuf(*)(const OIIO::ImageBuf &, const OIIO::ImageBuf &,
                             OIIO::ROI, int))&OIIO::ImageBufAlgo::over,
          "ImageBufAlgo_from_over");

  // zover()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &,
                    const OIIO::ImageBuf &, bool, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::zover,
          "ImageBufAlgo_zover");

  // reorient()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &,
                    int))&OIIO::ImageBufAlgo::reorient,
          "ImageBufAlgo_reorient");

  // rotate()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, float,
                    OIIO::Filter2D *, bool, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::rotate,
          "ImageBufAlgo_rotate");

  // rotate_()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, float, float,
                    float, OIIO::Filter2D *, bool, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::rotate,
          "ImageBufAlgo_rotate_around");

  // rotate90()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::rotate90,
          "ImageBufAlgo_rotate90");

  // rotate180()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::rotate180,
          "ImageBufAlgo_rotate180");

  // rotate270()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::rotate270,
          "ImageBufAlgo_rotate270");

  // compare()
  bbl::fn((OIIO::ImageBufAlgo::CompareResults(*)(
              const OIIO::ImageBuf &, const OIIO::ImageBuf &, float, float,
              OIIO::ROI, int))&OIIO::ImageBufAlgo::compare,
          "ImageBufAlgo_compare");

  // render_text()
  bbl::fn(
      (bool (*)(OIIO::ImageBuf &, int, int, OIIO::string_view, int,
                OIIO::string_view, OIIO::cspan<float>,
                OIIO::ImageBufAlgo::TextAlignX, OIIO::ImageBufAlgo::TextAlignY,
                int, OIIO::ROI, int))&OIIO::ImageBufAlgo::render_text,
      "ImageBufAlgo_render_text");

#if OIIO_VERSION >= OIIO_MAKE_VERSION(2, 5, 0)
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::string_view,
                    OIIO::string_view, bool, OIIO::string_view,
                    OIIO::string_view, const OIIO::ColorConfig *, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::colorconvert,
          "ImageBufAlgo_colorconvert");
#else
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::string_view,
                    OIIO::string_view, bool, OIIO::string_view,
                    OIIO::string_view, OIIO::ColorConfig *, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::colorconvert,
          "ImageBufAlgo_colorconvert");
#endif

  // resample()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, bool, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::resample,
          "ImageBufAlgo_resample");

  // resize()
  bbl::fn(
      (bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ParamValueSpan,
                OIIO::ROI, int))&OIIO::ImageBufAlgo::resize,
      "ImageBufAlgo_resize");

  // warp()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::M33fParam,
                    const OIIO::Filter2D *, bool, OIIO::ImageBuf::WrapMode,
                    OIIO::ROI, int))&OIIO::ImageBufAlgo::warp,
          "ImageBufAlgo_warp");

  // st_warp()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &,
                    const OIIO::ImageBuf &, const OIIO::Filter2D *, int, int,
                    bool, bool, OIIO::ROI, int))&OIIO::ImageBufAlgo::st_warp,
          "ImageBufAlgo_st_warp");

  // cut()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::cut,
          "ImageBufAlgo_cut");

  // invert()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::invert,
          "ImageBufAlgo_invert");

  // channel_sum()
  bbl::fn(
      (bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::cspan<float>,
                OIIO::ROI, int))&OIIO::ImageBufAlgo::channel_sum,
      "ImageBufAlgo_channel_sum");

  // contrast_remap()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &,
                    OIIO::cspan<float>, OIIO::cspan<float>, OIIO::cspan<float>,
                    OIIO::cspan<float>, OIIO::cspan<float>, OIIO::cspan<float>,
                    OIIO::ROI, int))&OIIO::ImageBufAlgo::contrast_remap,
          "ImageBufAlgo_contrast_remap");

  // saturate()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, float, int,
                    OIIO::ROI, int))&OIIO::ImageBufAlgo::saturate,
          "ImageBufAlgo_saturate");

  // unsharp_mask()
  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::string_view,
                    float, float, float, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::unsharp_mask,
          "ImageBufAlgo_unsharp_mask");

  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::premult,
          "ImageBufAlgo_premult");

  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::unpremult,
          "ImageBufAlgo_unpremult");

  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::repremult,
          "ImageBufAlgo_repremult");

  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &,
                    const OIIO::ImageBuf &, bool, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::convolve,
          "ImageBufAlgo_convolve");

  bbl::fn((OIIO::ImageBuf(*)(OIIO::string_view, float, float, float,
                             bool))&OIIO::ImageBufAlgo::make_kernel,
          "ImageBufAlgo_from_kernel");

  bbl::fn((std::string(*)(const OIIO::ImageBuf &, OIIO::string_view, OIIO::ROI,
                          int, int))&OIIO::ImageBufAlgo::computePixelHashSHA1,
          "ImageBufAlgo_computePixelHashSHA1");

  bbl::fn((bool (*)(OIIO::ImageBuf &, const OIIO::ImageBuf &, OIIO::ROI,
                    int))&OIIO::ImageBufAlgo::crop,
          "ImageBufAlgo_crop");
}
