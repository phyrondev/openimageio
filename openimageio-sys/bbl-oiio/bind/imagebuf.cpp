#include <babble>

#include <Imath/half.h>
#include <OpenImageIO/half.h>
#include <OpenImageIO/imagebuf.h>
#include <OpenImageIO/texture.h>
#include <sys/types.h>

namespace bblext {

/*auto ImageBuf_name(OIIO::ImageBuf const& buf) -> char const* {
    return buf.name().c_str();
}

auto ImageBuf_file_format_name(OIIO::ImageBuf const& buf) -> char const* {
    return buf.file_format_name().c_str();
}*/

bool ImageBuf_write(OIIO::ImageBuf const &buf,
                    OIIO::string_view const file_name) {
  return buf.write(file_name);
}

bool ImageBuf_write_with_spec(OIIO::ImageBuf const &buf,
                              OIIO::string_view const file_name,
                              OIIO::TypeDesc type_desc,
                              OIIO::string_view const file_format) {
  return buf.write(file_name, type_desc, file_format);
}

/// Workaround: if we pass in an OIIO::TypeDesc from Rust, we get an "invalid
/// memory reference" crash here.
/// TODO: figure out why and use ImageBuf::get_pixels() directly
bool ImageBuf_get_pixels(OIIO::ImageBuf const &buf, OIIO::ROI roi,
                         OIIO::TypeDesc::BASETYPE base_type, void *result) {
  OIIO::TypeDesc type_desc = OIIO::TypeDesc(base_type, 0);

  return buf.get_pixels(roi, type_desc, result);
}

bool ImageBuf_set_pixels_f64(OIIO::ImageBuf &buf, OIIO::ROI roi,
                             OIIO::cspan<double> pixels) {
  return buf.set_pixels(roi, pixels);
}

bool ImageBuf_set_pixels_f32(OIIO::ImageBuf &buf, OIIO::ROI roi,
                             OIIO::cspan<float> pixels) {
  return buf.set_pixels(roi, pixels);
}

bool ImageBuf_set_pixels_f16(OIIO::ImageBuf &buf, OIIO::ROI roi,
                             OIIO::cspan<half> pixels) {
  return buf.set_pixels(roi, pixels);
}

/*bool ImageBuf_set_pixels_u64(OIIO::ImageBuf &buf, OIIO::ROI roi,
                             OIIO::cspan<unsigned long> pixels) {
  return buf.set_pixels(roi, pixels);
}*/

bool ImageBuf_set_pixels_u32(OIIO::ImageBuf &buf, OIIO::ROI roi,
                             OIIO::cspan<u_int32_t> pixels) {
  return buf.set_pixels(roi, pixels);
}

bool ImageBuf_set_pixels_u16(OIIO::ImageBuf &buf, OIIO::ROI roi,
                             OIIO::cspan<u_int16_t> pixels) {
  return buf.set_pixels(roi, pixels);
}

bool ImageBuf_set_pixels_u8(OIIO::ImageBuf &buf, OIIO::ROI roi,
                            OIIO::cspan<u_int8_t> pixels) {
  return buf.set_pixels(roi, pixels);
}

void ImageBuf_expand_roi_full(OIIO::ImageBuf &buf) {
  buf.set_roi_full(buf.roi());
}

OIIO::ImageBuf ImageBuf_from_dimensions(int width, int height, int nchannels,
                                        OIIO::TypeDesc format,
                                        OIIO::string_view color_space) {
  OIIO::ImageSpec i_s = OIIO::ImageSpec(width, height, nchannels, format);
  i_s.set_colorspace(color_space);

  OIIO::ImageBuf i_b = OIIO::ImageBuf(i_s);
  return i_b;
}

} // namespace bblext

BBL_MODULE(oiio) {

  bbl::Class<OIIO::ImageBuf>()
      .ctor(bbl::Class<OIIO::ImageBuf>::Ctor<>(), "default")
      .ctor(bbl::Class<OIIO::ImageBuf>::Ctor<
                OIIO::string_view, int, int, std::shared_ptr<OIIO::ImageCache>,
                const OIIO::ImageSpec *, OIIO::Filesystem::IOProxy *>(
                "name", "subimage", "miplevel", "imagecache", "config",
                "ioproxy"),
            "ctor_01")
      .ctor(bbl::Class<OIIO::ImageBuf>::Ctor<OIIO::string_view,
                                             OIIO::ImageCache *>("name",
                                                                 "imagecache"),
            "ctor_02")
      .ctor(bbl::Class<OIIO::ImageBuf>::Ctor<const OIIO::ImageSpec &,
                                             OIIO::InitializePixels>("spec",
                                                                     "zero"),
            "ctor_03")
      .ctor(bbl::Class<OIIO::ImageBuf>::Ctor<OIIO::string_view,
                                             const OIIO::ImageSpec &,
                                             OIIO::InitializePixels>(
                "name", "spec", "zero"),
            "ctor_04")
      .ctor(bbl::Class<OIIO::ImageBuf>::Ctor<const OIIO::ImageSpec &, void *,
                                             OIIO::stride_t, OIIO::stride_t,
                                             OIIO::stride_t>(
                "spec", "buffer", "xstride", "ystride", "zstride"),
            "ctor_05")
      /* .ctor(bbl::Class<OIIO::ImageBuf>::Ctor<OIIO::string_view,
                                             const OIIO::ImageSpec &, void *>(
                "name", "spec", "buffer"),
            "ctor_06")*/
      .m(&OIIO::ImageBuf::clear)
      .m((void(OIIO::ImageBuf::*)()) & OIIO::ImageBuf::reset, "reset_00")
      //.m((void (OIIO::ImageBuf::*)(OIIO::string_view, OIIO::ImageCache *))
      //    &OIIO::ImageBuf::reset, "reset_01")
      //.m((void (OIIO::ImageBuf::*)(OIIO::string_view, int, int,
      // OIIO::ImageCache *, const OIIO::ImageSpec *, OIIO::Filesystem::IOProxy
      //*))
      //    &OIIO::ImageBuf::reset, "reset_02")
      .m((void(OIIO::ImageBuf::*)(const OIIO::ImageSpec &,
                                  OIIO::InitializePixels)) &
             OIIO::ImageBuf::reset,
         "reset_03")
      //.m((void (OIIO::ImageBuf::*)(OIIO::string_view, const OIIO::ImageSpec &,
      // OIIO::InitializePixels))
      //    &OIIO::ImageBuf::reset, "reset_04")
      .m((void(OIIO::ImageBuf::*)(const OIIO::ImageSpec &, void *,
                                  OIIO::stride_t, OIIO::stride_t,
                                  OIIO::stride_t)) &
             OIIO::ImageBuf::reset,
         "reset_05")
      .m(&OIIO::ImageBuf::make_writable)
      //.m((bool (OIIO::ImageBuf::*)(int, int, bool, OIIO::TypeDesc,
      // OIIO::ProgressCallback, void *))
      //     &OIIO::ImageBuf::read, "read_00")
      // .m((bool (OIIO::ImageBuf::*)(int, int, int, int, bool, OIIO::TypeDesc,
      // OIIO::ProgressCallback, void *))
      //     &OIIO::ImageBuf::read, "read_01")
      .m(&OIIO::ImageBuf::init_spec)
      //.m((bool (OIIO::ImageBuf::*)(OIIO::string_view, OIIO::TypeDesc,
      // OIIO::string_view, OIIO::ProgressCallback, void *) const)
      //     &OIIO::ImageBuf::write, "write_00")
      // .m((bool (OIIO::ImageBuf::*)(OIIO::string_view, OIIO::string_view,
      // OIIO::ProgressCallback, void *) const)
      //     &OIIO::ImageBuf::write, "write_01")
      // .m((bool (OIIO::ImageBuf::*)(ImageOutput *, OIIO::ProgressCallback,
      // void *) const)
      //     &OIIO::ImageBuf::write, "write_02")
      .m((void(OIIO::ImageBuf::*)(OIIO::cspan<OIIO::TypeDesc>)) &
             OIIO::ImageBuf::set_write_format,
         "set_write_format_01")
      .m(&OIIO::ImageBuf::set_write_tiles)
      /// TODO: rvalue reference
      .m(&OIIO::ImageBuf::set_write_ioproxy)
      .m((const OIIO::ImageBuf &(OIIO::ImageBuf::*)(const OIIO::ImageBuf &)) &
             OIIO::ImageBuf::operator=,
         "op_assign_00")
      .m((const OIIO::ImageBuf &(OIIO::ImageBuf::*)(OIIO::ImageBuf &&)) &
             OIIO::ImageBuf::operator=,
         "op_assign_01")
      .m(&OIIO::ImageBuf::copy_metadata)
      .m(&OIIO::ImageBuf::copy_pixels)
      .m((bool(OIIO::ImageBuf::*)(const OIIO::ImageBuf &, OIIO::TypeDesc)) &
             OIIO::ImageBuf::copy,
         "copy_00")
      .m((OIIO::ImageBuf(OIIO::ImageBuf::*)(OIIO::TypeDesc)
              const)&OIIO::ImageBuf::copy,
         "copy_01")
      .m(&OIIO::ImageBuf::swap)
      .m(&OIIO::ImageBuf::getchannel)
      .m((void(OIIO::ImageBuf::*)(int, int, int, float *, int,
                                  OIIO::ImageBuf::WrapMode) const) &
             OIIO::ImageBuf::getpixel,
         "getpixel")
      //.m(&OIIO::ImageBuf::interppixel)
      //.m(&OIIO::ImageBuf::interppixel_NDC)
      //.m(&OIIO::ImageBuf::interppixel_NDC_full)
      //.m(&OIIO::ImageBuf::interppixel_bicubic)
      //.m(&OIIO::ImageBuf::interppixel_bicubic_NDC)

      //.m(&OIIO::ImageBuf::get_pixels)
      .m(&OIIO::ImageBuf::initialized)
      .m(&OIIO::ImageBuf::storage)
      .m(&OIIO::ImageBuf::spec)
      .m(&OIIO::ImageBuf::specmod)
      .m(&OIIO::ImageBuf::nativespec)
      .m(&OIIO::ImageBuf::has_thumbnail)
      .m(&OIIO::ImageBuf::get_thumbnail)
      .m(&OIIO::ImageBuf::clear_thumbnail)
      .m(&OIIO::ImageBuf::name)
      .m(&OIIO::ImageBuf::file_format_name)
      .m(&OIIO::ImageBuf::subimage)
      .m(&OIIO::ImageBuf::nsubimages)
      .m(&OIIO::ImageBuf::miplevel)
      .m(&OIIO::ImageBuf::nmiplevels)
      .m(&OIIO::ImageBuf::nchannels)
      .m(&OIIO::ImageBuf::xbegin)
      .m(&OIIO::ImageBuf::xend)
      .m(&OIIO::ImageBuf::ybegin)
      .m(&OIIO::ImageBuf::yend)
      .m(&OIIO::ImageBuf::zbegin)
      .m(&OIIO::ImageBuf::zend)
      .m(&OIIO::ImageBuf::xmin)
      .m(&OIIO::ImageBuf::xmax)
      .m(&OIIO::ImageBuf::ymin)
      .m(&OIIO::ImageBuf::ymax)
      .m(&OIIO::ImageBuf::zmin)
      .m(&OIIO::ImageBuf::zmax)
      .m(&OIIO::ImageBuf::orientation)
      .m(&OIIO::ImageBuf::oriented_width)
      .m(&OIIO::ImageBuf::oriented_height)
      .m(&OIIO::ImageBuf::oriented_x)
      .m(&OIIO::ImageBuf::oriented_y)
      .m(&OIIO::ImageBuf::oriented_full_width)
      .m(&OIIO::ImageBuf::oriented_full_height)
      .m(&OIIO::ImageBuf::oriented_full_x)
      .m(&OIIO::ImageBuf::oriented_full_y)
      .m(&OIIO::ImageBuf::roi)
      .m(&OIIO::ImageBuf::roi_full)
      .m(&OIIO::ImageBuf::contains_roi)
      .m(&OIIO::ImageBuf::pixels_valid)
      .m(&OIIO::ImageBuf::pixeltype)
      .m((void *(OIIO::ImageBuf::*)()) & OIIO::ImageBuf::localpixels,
         "localpixels_00")
      .m((const void *(OIIO::ImageBuf::*)() const) &
             OIIO::ImageBuf::localpixels,
         "localpixels_01")
      .m(&OIIO::ImageBuf::pixel_stride)
      .m(&OIIO::ImageBuf::scanline_stride)
      .m(&OIIO::ImageBuf::z_stride)
      .m(&OIIO::ImageBuf::contiguous)
      .m(&OIIO::ImageBuf::cachedpixels)
      .m(&OIIO::ImageBuf::imagecache)
      .m((const void *(OIIO::ImageBuf::*)(int, int, int, int) const) &
             OIIO::ImageBuf::pixeladdr,
         "pixeladdr_00")
      .m((void *(OIIO::ImageBuf::*)(int, int, int, int)) &
             OIIO::ImageBuf::pixeladdr,
         "pixeladdr_01")
      .m(&OIIO::ImageBuf::pixelindex)
      .m((void(OIIO::ImageBuf::*)(int) const) & OIIO::ImageBuf::threads,
         "set_threads")
      .m((int(OIIO::ImageBuf::*)() const) & OIIO::ImageBuf::threads, "threads")
      .m(&OIIO::ImageBuf::has_error)
      .m(&OIIO::ImageBuf::geterror)
      .m(&OIIO::ImageBuf::deep)
      .m(&OIIO::ImageBuf::deep_samples)
      .m(&OIIO::ImageBuf::deep_pixel_ptr)
      .m(&OIIO::ImageBuf::deep_value)
      .m(&OIIO::ImageBuf::deep_value_uint)
      .m(&OIIO::ImageBuf::deep_insert_samples)
      .m(&OIIO::ImageBuf::deep_erase_samples)
      .m(&OIIO::ImageBuf::copy_deep_pixel)
      .m((OIIO::DeepData * (OIIO::ImageBuf::*)()) & OIIO::ImageBuf::deepdata,
         "deepdata")
      .m((const OIIO::DeepData *(OIIO::ImageBuf::*)() const) &
             OIIO::ImageBuf::deepdata,
         "deepdata_const")
      .m(bbl::Wrap(&OIIO::ImageBuf::WrapMode_from_string,
                   [](char const *name) -> OIIO::ImageBuf::WrapMode {
                     return OIIO::ImageBuf::WrapMode_from_string(name);
                   }))
      .m((void(OIIO::ImageBuf::*)(int, int, int, const float *, int)) &
             OIIO::ImageBuf::setpixel,
         "setpixel")
      .m(&OIIO::ImageBuf::set_deep_samples)
      .m(&OIIO::ImageBuf::set_thumbnail)
      .m(&OIIO::ImageBuf::set_orientation)
      .m(&OIIO::ImageBuf::set_roi_full)
      .m(&OIIO::ImageBuf::set_origin)
      .m(&OIIO::ImageBuf::set_full)
      //.m(&OIIO::ImageBuf::set_pixels)
      .m((void(OIIO::ImageBuf::*)(int, int, int, int, int, float)) &
             OIIO::ImageBuf::set_deep_value,
         "set_deep_value_00")
      .m((void(OIIO::ImageBuf::*)(int, int, int, int, int, uint32_t)) &
             OIIO::ImageBuf::set_deep_value,
         "set_deep_value_01");

  // bbl::fn(&bblext::ImageBuf_name);
  // bbl::fn(&bblext::ImageBuf_file_format_name);
  bbl::fn(&bblext::ImageBuf_write);
  bbl::fn(&bblext::ImageBuf_write_with_spec);
  bbl::fn(&bblext::ImageBuf_get_pixels);
  // bbl::fn(&bblext::ImageBuf_set_pixels_f16);
  bbl::fn(&bblext::ImageBuf_set_pixels_f32);
  bbl::fn(&bblext::ImageBuf_set_pixels_f64);
  // bbl::fn(&bblext::ImageBuf_set_pixels_u64);
  bbl::fn(&bblext::ImageBuf_set_pixels_u32);
  bbl::fn(&bblext::ImageBuf_set_pixels_u16);
  bbl::fn(&bblext::ImageBuf_set_pixels_u8);
  bbl::fn(&bblext::ImageBuf_expand_roi_full);

  // FIXME: why can we not call ImageSpec::from_dimensions and have it work?
  bbl::fn(&bblext::ImageBuf_from_dimensions);

  bbl::Class<std::shared_ptr<OIIO::ImageBuf>>("ImageBufSharedPtr")
      .smartptr_to<OIIO::ImageBuf>();

  bbl::Enum<OIIO::ImageBuf::IBStorage>();
  bbl::Enum<OIIO::ImageBuf::WrapMode>();

  bbl::Class<OIIO::ImageBuf::IteratorBase>()
      // .ctor(bbl::Class<OIIO::ImageBuf::IteratorBase>::Ctor<const
      // OIIO::ImageBuf &, OIIO::ImageBuf::WrapMode, bool>("ib", "wrap",
      // "write"), "ctor_00")
      // .ctor(bbl::Class<OIIO::ImageBuf::IteratorBase>::Ctor<const
      // OIIO::ImageBuf &, const OIIO::ROI &, OIIO::ImageBuf::WrapMode>("ib",
      // "roi", "wrap"), "ctor_01")
      // .ctor(bbl::Class<OIIO::ImageBuf::IteratorBase>::Ctor<const
      // OIIO::ImageBuf &, int, int, int, int, int, int,
      // OIIO::ImageBuf::WrapMode>("ib", "xbegin", "xend", "ybegin", "yend",
      // "zbegin", "zend", "wrap"), "ctor_02")
      // .m(&OIIO::ImageBuf::IteratorBase::assign_base)
      .m(&OIIO::ImageBuf::IteratorBase::x)
      .m(&OIIO::ImageBuf::IteratorBase::y)
      .m(&OIIO::ImageBuf::IteratorBase::z)
      .m((bool(OIIO::ImageBuf::IteratorBase::*)() const) &
             OIIO::ImageBuf::IteratorBase::valid,
         "valid_00")
      .m((bool(OIIO::ImageBuf::IteratorBase::*)(int, int, int) const) &
             OIIO::ImageBuf::IteratorBase::valid,
         "valid_01")
      .m((bool(OIIO::ImageBuf::IteratorBase::*)(int, int, int) const) &
             OIIO::ImageBuf::IteratorBase::exists,
         "exists_00")
      .m((bool(OIIO::ImageBuf::IteratorBase::*)() const) &
             OIIO::ImageBuf::IteratorBase::exists,
         "exists_01")
      .m(&OIIO::ImageBuf::IteratorBase::done)
      .m(&OIIO::ImageBuf::IteratorBase::deep_samples)
      .m(&OIIO::ImageBuf::IteratorBase::wrap)
      .m(&OIIO::ImageBuf::IteratorBase::pos)
      .m((void(OIIO::ImageBuf::IteratorBase::*)()) &
             OIIO::ImageBuf::IteratorBase::operator++,
         "op_inc_00")
      .m((void(OIIO::ImageBuf::IteratorBase::*)(int)) &
             OIIO::ImageBuf::IteratorBase::operator++,
         "op_inc_01")
      .m(&OIIO::ImageBuf::IteratorBase::range)
      .m(&OIIO::ImageBuf::IteratorBase::rerange);

  // #if 0
  /// TODO: instantiate this template
  bbl::Class<OIIO::ImageBuf::Iterator<float, float>>()
      .ctor(bbl::Class<OIIO::ImageBuf::Iterator<float, float>>::Ctor<
                OIIO::ImageBuf &, OIIO::ImageBuf::WrapMode>("ib", "wrap"),
            "ctor_00")
      .ctor(bbl::Class<OIIO::ImageBuf::Iterator<float, float>>::Ctor<
                OIIO::ImageBuf &, int, int, int, OIIO::ImageBuf::WrapMode>(
                "ib", "x", "y", "z", "wrap"),
            "ctor_01")
      .ctor(bbl::Class<OIIO::ImageBuf::Iterator<float, float>>::Ctor<
                OIIO::ImageBuf &, const OIIO::ROI &, OIIO::ImageBuf::WrapMode>(
                "ib", "roi", "wrap"),
            "ctor_02")
      .ctor(bbl::Class<OIIO::ImageBuf::Iterator<float, float>>::Ctor<
                OIIO::ImageBuf &, int, int, int, int, int, int,
                OIIO::ImageBuf::WrapMode>("ib", "xbegin", "xend", "ybegin",
                                          "yend", "zbegin", "zend", "wrap"),
            "ctor_03")
      .m(&OIIO::ImageBuf::Iterator<float, float>::operator=, "op_assign")
      .m(&OIIO::ImageBuf::Iterator<float, float>::operator*, "op_mul")
      .m((float(OIIO::ImageBuf::Iterator<float, float>::*)(int) const) &
             OIIO::ImageBuf::Iterator<float, float>::operator[],
         "op_index_ref")
      .m((OIIO::DataProxy<float, float>(
             OIIO::ImageBuf::Iterator<float, float>::*)(int)) &
             OIIO::ImageBuf::Iterator<float, float>::operator[],
         "op_index_mut")
      .m(&OIIO::ImageBuf::Iterator<float, float>::rawptr)
      .m(&OIIO::ImageBuf::Iterator<float, float>::set_deep_samples)
      .m(&OIIO::ImageBuf::Iterator<float, float>::deep_value)
      .m(&OIIO::ImageBuf::Iterator<float, float>::deep_value_uint)
      .m((void(OIIO::ImageBuf::Iterator<float, float>::*)(int, int, float)) &
             OIIO::ImageBuf::Iterator<float, float>::set_deep_value,
         "set_deep_value_00")
      .m((void(OIIO::ImageBuf::Iterator<float, float>::*)(int, int, uint32_t)) &
             OIIO::ImageBuf::Iterator<float, float>::set_deep_value,
         "set_deep_value_01");
  // #endif

#if 0
    /// TODO: instantiate this template
    bbl::Class<OIIO::ImageBuf::ConstIterator>()
        .ctor(bbl::Class<OIIO::ImageBuf::ConstIterator>::Ctor<const ImageBuf &, WrapMode>("ib", "wrap"), "ctor_00")
        .ctor(bbl::Class<OIIO::ImageBuf::ConstIterator>::Ctor<const ImageBuf &, int, int, int, WrapMode>("ib", "x_", "y_", "z_", "wrap"), "ctor_01")
        .ctor(bbl::Class<OIIO::ImageBuf::ConstIterator>::Ctor<const ImageBuf &, const ROI &, WrapMode>("ib", "roi", "wrap"), "ctor_02")
        .ctor(bbl::Class<OIIO::ImageBuf::ConstIterator>::Ctor<const ImageBuf &, int, int, int, int, int, int, WrapMode>("ib", "xbegin", "xend", "ybegin", "yend", "zbegin", "zend", "wrap"), "ctor_03")
        .m(&OIIO::ImageBuf::ConstIterator::operator=, "op_assign")
        .m(&OIIO::ImageBuf::ConstIterator::operator*, "op_mul")
        .m(&OIIO::ImageBuf::ConstIterator::operator[], "op_index")
        .m(&OIIO::ImageBuf::ConstIterator::rawptr)
        .m(&OIIO::ImageBuf::ConstIterator::deep_value)
        .m(&OIIO::ImageBuf::ConstIterator::deep_value_uint)
    ;
#endif

  bbl::Enum<OIIO::InitializePixels>();
}
