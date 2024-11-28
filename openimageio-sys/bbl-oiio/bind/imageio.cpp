#include <babble>

#include <OpenImageIO/imagebuf.h>

// bindfile

BBL_MODULE(oiio) {

  bbl::Class<OIIO::ROI>()
      .value_type()
      .f(&OIIO::ROI::xbegin)
      .f(&OIIO::ROI::xend)
      .f(&OIIO::ROI::ybegin)
      .f(&OIIO::ROI::yend)
      .f(&OIIO::ROI::zbegin)
      .f(&OIIO::ROI::zend)
      .f(&OIIO::ROI::chbegin)
      .f(&OIIO::ROI::chend)
      .ctor(bbl::Class<OIIO::ROI>::Ctor<>(), "default")
      .ctor(bbl::Class<OIIO::ROI>::Ctor<int, int, int, int, int, int, int, int>(
                "xbegin", "xend", "ybegin", "yend", "zbegin", "zend", "chbegin",
                "chend"),
            "with_dimensions")
      .m(&OIIO::ROI::defined)
      .m(&OIIO::ROI::width)
      .m(&OIIO::ROI::height)
      .m(&OIIO::ROI::depth)
      .m(&OIIO::ROI::nchannels)
      .m(&OIIO::ROI::npixels)
      .m(&OIIO::ROI::All)
      .m((bool(OIIO::ROI::*)(int, int, int, int) const) & OIIO::ROI::contains,
         "contains_region")
      .m((bool(OIIO::ROI::*)(const OIIO::ROI &) const) & OIIO::ROI::contains,
         "contains");

  bbl::Class<OIIO::ImageInput::unique_ptr>("ImageInputPtr")
      .smartptr_to<OIIO::ImageInput>();

  bbl::Class<OIIO::ImageInput>()
      .m(bbl::Wrap((OIIO::ImageInput::unique_ptr(*)(
                       const std::string &, const OIIO::ImageSpec *,
                       OIIO::Filesystem::IOProxy *))&OIIO::ImageInput::open,
                   [](char const *filename, OIIO::ImageSpec const *spec,
                      OIIO::Filesystem::IOProxy *ioproxy)
                       -> OIIO::ImageInput::unique_ptr {
                     return OIIO::ImageInput::open(filename, spec, ioproxy);
                   }),
         "open_with_ioproxy")
      .m(bbl::Wrap((bool(OIIO::ImageInput::*)(const std::string &,
                                              OIIO::ImageSpec &)) &
                       OIIO::ImageInput::open,
                   [](OIIO::ImageInput &_this, char const *filename,
                      OIIO::ImageSpec &spec) -> bool {
                     return _this.open(filename, spec);
                   }),
         "open")
      .m(bbl::Wrap(
             (bool(OIIO::ImageInput::*)(const std::string &, OIIO::ImageSpec &,
                                        const OIIO::ImageSpec &)) &
                 OIIO::ImageInput::open,
             [](OIIO::ImageInput &_this, char const *filename,
                OIIO::ImageSpec &spec, OIIO::ImageSpec const &config) -> bool {
               return _this.open(filename, spec, config);
             }),
         "open_with_config")
      .m(bbl::Wrap(
             (OIIO::ImageInput::unique_ptr(*)(
                 OIIO::string_view, bool, const OIIO::ImageSpec *,
                 OIIO::Filesystem::IOProxy *,
                 OIIO::string_view))&OIIO::ImageInput::create,
             [](char const *filename, bool doopen,
                OIIO::ImageSpec const *config,
                OIIO::Filesystem::IOProxy *ioproxy,
                char const *plugin_searchpath) -> OIIO::ImageInput::unique_ptr {
               return OIIO::ImageInput::create(filename, doopen, config,
                                               ioproxy, plugin_searchpath);
             }),
         "create_with_ioproxy")
      //.m(&OIIO::ImageInput::destroy)
      .m(&OIIO::ImageInput::format_name)
      .m(bbl::Wrap(&OIIO::ImageInput::supports,
                   [](OIIO::ImageInput const &_this, char const *feature)
                       -> int { return _this.supports(feature); }))
      .m(bbl::Wrap((bool(OIIO::ImageInput::*)(const std::string &) const) &
                       OIIO::ImageInput::valid_file,
                   [](OIIO::ImageInput const &_this, char const *filename)
                       -> bool { return _this.valid_file(filename); }),
         "valid_file")
      .m((const OIIO::ImageSpec &(OIIO::ImageInput::*)() const) &
             OIIO::ImageInput::spec,
         "spec")
      .m((OIIO::ImageSpec(OIIO::ImageInput::*)(int,
                                               int))&OIIO::ImageInput::spec,
         "spec_from_subimage")
      .m(&OIIO::ImageInput::spec_dimensions)
      .m(&OIIO::ImageInput::get_thumbnail)
      .m(&OIIO::ImageInput::close)
      .m(&OIIO::ImageInput::current_subimage)
      .m(&OIIO::ImageInput::current_miplevel)
      .m((bool(OIIO::ImageInput::*)(int, int)) &
             OIIO::ImageInput::seek_subimage,
         "seek_subimage_00")
      //.m((bool(OIIO::ImageInput::*)(int, int, OIIO::ImageSpec &)) &
      //       OIIO::ImageInput::seek_subimage,
      //   "seek_subimage_01")
      //.m((bool(OIIO::ImageInput::*)(int, OIIO::ImageSpec &)) &
      //       OIIO::ImageInput::seek_subimage,
      //   "seek_subimage_02")
      .m((bool(OIIO::ImageInput::*)(int, int, OIIO::TypeDesc, void *,
                                    OIIO::stride_t)) &
             OIIO::ImageInput::read_scanline,
         "read_scanline_00")
      .m((bool(OIIO::ImageInput::*)(int, int, float *)) &
             OIIO::ImageInput::read_scanline,
         "read_scanline_01")
      .m((bool(OIIO::ImageInput::*)(int, int, int, int, int, int, int,
                                    OIIO::TypeDesc, void *, OIIO::stride_t,
                                    OIIO::stride_t)) &
             OIIO::ImageInput::read_scanlines,
         "read_scanlines")
      .m((bool(OIIO::ImageInput::*)(int, int, int, OIIO::TypeDesc, void *,
                                    OIIO::stride_t, OIIO::stride_t,
                                    OIIO::stride_t)) &
             OIIO::ImageInput::read_tile,
         "read_tile_00")
      .m((bool(OIIO::ImageInput::*)(int, int, int, float *)) &
             OIIO::ImageInput::read_tile,
         "read_tile_01")
      .m((bool(OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, int,
                                    int, OIIO::TypeDesc, void *, OIIO::stride_t,
                                    OIIO::stride_t, OIIO::stride_t)) &
             OIIO::ImageInput::read_tiles,
         "read_tiles")
      // .m((bool (OIIO::ImageInput::*)(int, int, int, int, OIIO::TypeDesc, void
      // *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, ProgressCallback,
      // void *))
      //     &OIIO::ImageInput::read_image, "read_image_00")
      // .m((bool (OIIO::ImageInput::*)(OIIO::TypeDesc, void *, OIIO::stride_t,
      // OIIO::stride_t, OIIO::stride_t, ProgressCallback, void *))
      //     &OIIO::ImageInput::read_image, "read_image_01")
      // .m((bool (OIIO::ImageInput::*)(int, int, OIIO::TypeDesc, void *,
      // OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, ProgressCallback, void
      // *))
      //     &OIIO::ImageInput::read_image, "read_image_02")
      .m((bool(OIIO::ImageInput::*)(int, int, int, int, int, int, int,
                                    OIIO::DeepData &)) &
             OIIO::ImageInput::read_native_deep_scanlines,
         "read_native_deep_scanlines")
      .m((bool(OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, int,
                                    int, OIIO::DeepData &)) &
             OIIO::ImageInput::read_native_deep_tiles,
         "read_native_deep_tiles")
      .m((bool(OIIO::ImageInput::*)(int, int, OIIO::DeepData &)) &
             OIIO::ImageInput::read_native_deep_image,
         "read_native_deep_image")
      .m(&OIIO::ImageInput::read_native_scanline)
      .m((bool(OIIO::ImageInput::*)(int, int, int, int, int, void *)) &
             OIIO::ImageInput::read_native_scanlines,
         "read_native_scanlines_00")
      .m((bool(OIIO::ImageInput::*)(int, int, int, int, int, int, int,
                                    void *)) &
             OIIO::ImageInput::read_native_scanlines,
         "read_native_scanlines_01")
      .m(&OIIO::ImageInput::read_native_tile)
      .m((bool(OIIO::ImageInput::*)(int, int, int, int, int, int, int, int,
                                    void *)) &
             OIIO::ImageInput::read_native_tiles,
         "read_native_tiles_00")
      .m((bool(OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, int,
                                    int, void *)) &
             OIIO::ImageInput::read_native_tiles,
         "read_native_tiles_01")
      .m(&OIIO::ImageInput::set_ioproxy)
      .m(&OIIO::ImageInput::has_error)
      .m(&OIIO::ImageInput::geterror)
      .m((void(OIIO::ImageInput::*)(int)) & OIIO::ImageInput::threads,
         "threads")
      .m((int(OIIO::ImageInput::*)() const) & OIIO::ImageInput::threads,
         "threads_const")
      .m(&OIIO::ImageInput::lock)
      .m(&OIIO::ImageInput::unlock)
      .m(&OIIO::ImageInput::try_lock);

  bbl::Class<OIIO::ImageOutput::unique_ptr>("ImageOutputPtr")
      .smartptr_to<OIIO::ImageOutput>();

  bbl::Class<OIIO::ImageOutput>()
      .m(bbl::Wrap((OIIO::ImageOutput::unique_ptr(*)(
                       OIIO::string_view, OIIO::Filesystem::IOProxy *,
                       OIIO::string_view))&OIIO::ImageOutput::create,
                   [](char const *filename, OIIO::Filesystem::IOProxy *ioproxy,
                      char const *plugin_searchpath)
                       -> OIIO::ImageOutput::unique_ptr {
                     return OIIO::ImageOutput::create(filename, ioproxy,
                                                      plugin_searchpath);
                   }),
         "create")
      //.m(&OIIO::ImageOutput::destroy)
      .m(&OIIO::ImageOutput::format_name)
      .m(bbl::Wrap(&OIIO::ImageOutput::supports,
                   [](OIIO::ImageOutput const &_this, char const *feature)
                       -> int { return _this.supports(feature); }))
      .m(bbl::Wrap((bool(OIIO::ImageOutput::*)(const std::string &,
                                               const OIIO::ImageSpec &,
                                               OIIO::ImageOutput::OpenMode)) &
                       OIIO::ImageOutput::open,
                   [](OIIO::ImageOutput &_this, char const *filename,
                      OIIO::ImageSpec const &spec,
                      OIIO::ImageOutput::OpenMode openmode) -> bool {
                     return _this.open(filename, spec, openmode);
                   }),
         "open")
      .m(bbl::Wrap((bool(OIIO::ImageOutput::*)(const std::string &, int,
                                               const OIIO::ImageSpec *)) &
                       OIIO::ImageOutput::open,
                   [](OIIO::ImageOutput &_this, char const *filename,
                      int num_subimages, OIIO::ImageSpec const *specs) -> bool {
                     return _this.open(filename, num_subimages, specs);
                   }),
         "open_multi_subimage")
      .m(&OIIO::ImageOutput::spec)
      .m(&OIIO::ImageOutput::close)
      .m(&OIIO::ImageOutput::write_scanline)
      .m(&OIIO::ImageOutput::write_scanlines)
      .m(&OIIO::ImageOutput::write_tile)
      .m(&OIIO::ImageOutput::write_tiles)
      .m(&OIIO::ImageOutput::write_rectangle)
      // TODO: ProgressCallback
      // .m(&OIIO::ImageOutput::write_image)
      .m(&OIIO::ImageOutput::write_deep_scanlines)
      .m(&OIIO::ImageOutput::write_deep_tiles)
      .m(&OIIO::ImageOutput::write_deep_image)
      .m(&OIIO::ImageOutput::set_thumbnail)
      .m(&OIIO::ImageOutput::copy_image)
      .m(&OIIO::ImageOutput::set_ioproxy)
      .m(&OIIO::ImageOutput::has_error)
      .m(&OIIO::ImageOutput::geterror)
      .m((void(OIIO::ImageOutput::*)(int)) & OIIO::ImageOutput::threads,
         "setthreads")
      .m((int(OIIO::ImageOutput::*)() const) & OIIO::ImageOutput::threads,
         "getthreads");

  bbl::Enum<OIIO::ImageOutput::OpenMode>();

  bbl::fn(&OIIO::roi_union);

  bbl::fn(&OIIO::roi_intersection);

  bbl::ClassIncomplete<OIIO::Filesystem::IOProxy>();
}
