#include <babble>

#include <OpenImageIO/imagebuf.h>

BBL_MODULE(oiio) {

    bbl::Class<OIIO::ROI>()
        .ctor(bbl::Class<OIIO::ROI>::Ctor<>(), "default")
        .ctor(bbl::Class<OIIO::ROI>::Ctor<int, int, int, int, int, int, int, int>("xbegin", "xend", "ybegin", "yend", "zbegin", "zend", "chbegin", "chend"), "with_dimensions")
        .m(&OIIO::ROI::defined)
        .m(&OIIO::ROI::width)
        .m(&OIIO::ROI::height)
        .m(&OIIO::ROI::depth)
        .m(&OIIO::ROI::nchannels)
        .m(&OIIO::ROI::npixels)
        .m(&OIIO::ROI::All)
        .m((bool (OIIO::ROI::*)(int, int, int, int) const)
            &OIIO::ROI::contains, "contains_region")
        .m((bool (OIIO::ROI::*)(const OIIO::ROI &) const)
            &OIIO::ROI::contains, "contains")
    ;

    bbl::Class<OIIO::ImageSpec>()
        .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<OIIO::TypeDesc>("format"), "new")
        // .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<string_view>("format"), "ctor_01")
        .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<int, int, int, OIIO::TypeDesc>("xres", "yres", "nchans", "fmt"), "with_dimensions")
        // .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<int, int, int, string_view>("xres", "yres", "nchans", "fmt"), "ctor_03")
        .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<const OIIO::ROI &, OIIO::TypeDesc>("roi", "fmt"), "with_region")
        // .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<const OIIO::ROI &, string_view>("roi", "fmt"), "ctor_05")
        .m((void (OIIO::ImageSpec::*)(OIIO::TypeDesc))
            &OIIO::ImageSpec::set_format, "set_format")
        // .m((void (OIIO::ImageSpec::*)(string_view))
        //     &OIIO::ImageSpec::set_format, "set_format_01")
        .m(&OIIO::ImageSpec::default_channel_names)
        .m((size_t (OIIO::ImageSpec::*)() const)
            &OIIO::ImageSpec::channel_bytes, "channel_bytes_00")
        .m((size_t (OIIO::ImageSpec::*)(int, bool) const)
            &OIIO::ImageSpec::channel_bytes, "channel_bytes_01")
        .m((size_t (OIIO::ImageSpec::*)(bool) const)
            &OIIO::ImageSpec::pixel_bytes, "pixel_bytes_00")
        .m((size_t (OIIO::ImageSpec::*)(int, int, bool) const)
            &OIIO::ImageSpec::pixel_bytes, "pixel_bytes_01")
        .m(&OIIO::ImageSpec::scanline_bytes)
        .m(&OIIO::ImageSpec::tile_pixels)
        .m(&OIIO::ImageSpec::tile_bytes)
        .m(&OIIO::ImageSpec::image_pixels)
        .m(&OIIO::ImageSpec::image_bytes)
        .m(&OIIO::ImageSpec::size_t_safe)
        .m((void (*)(OIIO::stride_t &, OIIO::stride_t &, OIIO::stride_t &, OIIO::stride_t, int, int, int))
            &OIIO::ImageSpec::auto_stride, "auto_stride_00")
        .m((void (*)(OIIO::stride_t &, OIIO::stride_t &, OIIO::stride_t &, OIIO::TypeDesc, int, int, int))
            &OIIO::ImageSpec::auto_stride, "auto_stride_01")
        .m((void (*)(OIIO::stride_t &, OIIO::TypeDesc, int))
            &OIIO::ImageSpec::auto_stride, "auto_stride_02")
        // .m((void (OIIO::ImageSpec::*)(string_view, OIIO::TypeDesc, const void *))
        //     &OIIO::ImageSpec::attribute, "attribute_00")
        // .m((void (OIIO::ImageSpec::*)(string_view, unsigned int))
        //     &OIIO::ImageSpec::attribute, "attribute_01")
        // .m((void (OIIO::ImageSpec::*)(string_view, int))
        //     &OIIO::ImageSpec::attribute, "attribute_02")
        // .m((void (OIIO::ImageSpec::*)(string_view, float))
        //     &OIIO::ImageSpec::attribute, "attribute_03")
        // .m((void (OIIO::ImageSpec::*)(string_view, string_view))
        //     &OIIO::ImageSpec::attribute, "attribute_04")
        // .m((void (OIIO::ImageSpec::*)(string_view, OIIO::TypeDesc, string_view))
        //     &OIIO::ImageSpec::attribute, "attribute_05")
        // .m(&OIIO::ImageSpec::erase_attribute)
        // .m((ParamValue * (OIIO::ImageSpec::*)(string_view, OIIO::TypeDesc, bool))
        //     &OIIO::ImageSpec::find_attribute, "find_attribute_00")
        // .m((const ParamValue * (OIIO::ImageSpec::*)(string_view, OIIO::TypeDesc, bool) const)
        //     &OIIO::ImageSpec::find_attribute, "find_attribute_01")
        // .m((const ParamValue * (OIIO::ImageSpec::*)(string_view, ParamValue &, OIIO::TypeDesc, bool) const)
        //     &OIIO::ImageSpec::find_attribute, "find_attribute_02")
        .m(&OIIO::ImageSpec::getattributetype)
        .m(&OIIO::ImageSpec::getattribute)
        .m(&OIIO::ImageSpec::get_int_attribute)
        .m(&OIIO::ImageSpec::get_float_attribute)
        .m(&OIIO::ImageSpec::get_string_attribute)
        .m(&OIIO::ImageSpec::metadata_val)
        .m(&OIIO::ImageSpec::serialize)
        .m(&OIIO::ImageSpec::to_xml)
        .m(&OIIO::ImageSpec::from_xml)
        .m(&OIIO::ImageSpec::decode_compression_metadata)
        .m(&OIIO::ImageSpec::valid_tile_range)
        .m(&OIIO::ImageSpec::channelformat)
        .m(&OIIO::ImageSpec::channel_name)
        .m(&OIIO::ImageSpec::get_channelformats)
        .m(&OIIO::ImageSpec::channelindex)
        .m(&OIIO::ImageSpec::roi)
        .m(&OIIO::ImageSpec::roi_full)
        .m(&OIIO::ImageSpec::set_roi)
        .m(&OIIO::ImageSpec::set_roi_full)
        .m(&OIIO::ImageSpec::copy_dimensions)
        .m(&OIIO::ImageSpec::undefined)
        // .m((AttrDelegate<ImageSpec> (OIIO::ImageSpec::*)(string_view))
        //     &OIIO::ImageSpec::operator[], "op_index_00")
        // .m((AttrDelegate<const ImageSpec> (OIIO::ImageSpec::*)(string_view) const)
        //     &OIIO::ImageSpec::operator[], "op_index_01")
        // .m((ImageSpec & (OIIO::ImageSpec::*)(const ImageSpec &))
        //     &OIIO::ImageSpec::operator=, "op_assign_00")
        // .m((ImageSpec & (OIIO::ImageSpec::*)(ImageSpec &&))
        //     &OIIO::ImageSpec::operator=, "op_assign_01")
    ;

    bbl::Class<OIIO::ImageInput::unique_ptr>("ImageInputPtr")
        .smartptr_to<OIIO::ImageInput>()
    ;

    bbl::Class<OIIO::ImageInput>()
        .m((OIIO::ImageInput::unique_ptr (*)(const std::string &, const OIIO::ImageSpec *, OIIO::Filesystem::IOProxy *))
            &OIIO::ImageInput::open, "open_00")
        .m((OIIO::ImageInput::unique_ptr (*)(const std::wstring &, const OIIO::ImageSpec *, OIIO::Filesystem::IOProxy *))
            &OIIO::ImageInput::open, "open_01")
        .m((bool (OIIO::ImageInput::*)(const std::string &, OIIO::ImageSpec &))
            &OIIO::ImageInput::open, "open_02")
        .m((bool (OIIO::ImageInput::*)(const std::wstring &, OIIO::ImageSpec &))
            &OIIO::ImageInput::open, "open_03")
        .m((bool (OIIO::ImageInput::*)(const std::string &, OIIO::ImageSpec &, const OIIO::ImageSpec &))
            &OIIO::ImageInput::open, "open_04")
        .m((bool (OIIO::ImageInput::*)(const std::wstring &, OIIO::ImageSpec &, const OIIO::ImageSpec &))
            &OIIO::ImageInput::open, "open_05")
        // .m((OIIO::ImageInput::unique_ptr (*)(string_view, bool, const OIIO::ImageSpec *, OIIO::Filesystem::IOProxy *, string_view))
        //     &OIIO::ImageInput::create, "create_00")
        // .m((OIIO::ImageInput::unique_ptr (*)(const std::wstring &, bool, const OIIO::ImageSpec *, OIIO::Filesystem::IOProxy *, string_view))
        //     &OIIO::ImageInput::create, "create_01")
        // .m((OIIO::ImageInput::unique_ptr (*)(const std::string &, bool, const OIIO::ImageSpec *, string_view))
        //     &OIIO::ImageInput::create, "create_02")
        // .m((OIIO::ImageInput::unique_ptr (*)(const std::string &, const std::string &))
        //     &OIIO::ImageInput::create, "create_03")
        .m(&OIIO::ImageInput::destroy)
        .m(&OIIO::ImageInput::format_name)
        .m(&OIIO::ImageInput::supports)
        .m((bool (OIIO::ImageInput::*)(const std::string &) const)
            &OIIO::ImageInput::valid_file, "valid_file_00")
        .m((bool (OIIO::ImageInput::*)(const std::wstring &) const)
            &OIIO::ImageInput::valid_file, "valid_file_01")
        .m((const OIIO::ImageSpec & (OIIO::ImageInput::*)() const)
            &OIIO::ImageInput::spec, "spec_00")
        .m((OIIO::ImageSpec (OIIO::ImageInput::*)(int, int))
            &OIIO::ImageInput::spec, "spec_01")
        .m(&OIIO::ImageInput::spec_dimensions)
        .m(&OIIO::ImageInput::get_thumbnail)
        .m(&OIIO::ImageInput::close)
        .m(&OIIO::ImageInput::current_subimage)
        .m(&OIIO::ImageInput::current_miplevel)
        .m((bool (OIIO::ImageInput::*)(int, int))
            &OIIO::ImageInput::seek_subimage, "seek_subimage_00")
        .m((bool (OIIO::ImageInput::*)(int, int, OIIO::ImageSpec &))
            &OIIO::ImageInput::seek_subimage, "seek_subimage_01")
        .m((bool (OIIO::ImageInput::*)(int, OIIO::ImageSpec &))
            &OIIO::ImageInput::seek_subimage, "seek_subimage_02")
        .m((bool (OIIO::ImageInput::*)(int, int, OIIO::TypeDesc, void *, OIIO::stride_t))
            &OIIO::ImageInput::read_scanline, "read_scanline_00")
        .m((bool (OIIO::ImageInput::*)(int, int, float *))
            &OIIO::ImageInput::read_scanline, "read_scanline_01")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t))
            &OIIO::ImageInput::read_scanlines, "read_scanlines_00")
        .m((bool (OIIO::ImageInput::*)(int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t))
            &OIIO::ImageInput::read_scanlines, "read_scanlines_01")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t))
            &OIIO::ImageInput::read_scanlines, "read_scanlines_02")
        .m((bool (OIIO::ImageInput::*)(int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t))
            &OIIO::ImageInput::read_tile, "read_tile_00")
        .m((bool (OIIO::ImageInput::*)(int, int, int, float *))
            &OIIO::ImageInput::read_tile, "read_tile_01")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t))
            &OIIO::ImageInput::read_tiles, "read_tiles_00")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t))
            &OIIO::ImageInput::read_tiles, "read_tiles_01")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t))
            &OIIO::ImageInput::read_tiles, "read_tiles_02")
        // .m((bool (OIIO::ImageInput::*)(int, int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, ProgressCallback, void *))
        //     &OIIO::ImageInput::read_image, "read_image_00")
        // .m((bool (OIIO::ImageInput::*)(OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, ProgressCallback, void *))
        //     &OIIO::ImageInput::read_image, "read_image_01")
        // .m((bool (OIIO::ImageInput::*)(int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, ProgressCallback, void *))
        //     &OIIO::ImageInput::read_image, "read_image_02")
        .m((bool (OIIO::ImageInput::*)(float *))
            &OIIO::ImageInput::read_image, "read_image_03")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, OIIO::DeepData &))
            &OIIO::ImageInput::read_native_deep_scanlines, "read_native_deep_scanlines_00")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, OIIO::DeepData &))
            &OIIO::ImageInput::read_native_deep_scanlines, "read_native_deep_scanlines_01")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, int, int, OIIO::DeepData &))
            &OIIO::ImageInput::read_native_deep_tiles, "read_native_deep_tiles_00")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, OIIO::DeepData &))
            &OIIO::ImageInput::read_native_deep_tiles, "read_native_deep_tiles_01")
        .m((bool (OIIO::ImageInput::*)(int, int, OIIO::DeepData &))
            &OIIO::ImageInput::read_native_deep_image, "read_native_deep_image_00")
        .m((bool (OIIO::ImageInput::*)(OIIO::DeepData &))
            &OIIO::ImageInput::read_native_deep_image, "read_native_deep_image_01")
        .m(&OIIO::ImageInput::read_native_scanline)
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, void *))
            &OIIO::ImageInput::read_native_scanlines, "read_native_scanlines_00")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, void *))
            &OIIO::ImageInput::read_native_scanlines, "read_native_scanlines_01")
        .m(&OIIO::ImageInput::read_native_tile)
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, void *))
            &OIIO::ImageInput::read_native_tiles, "read_native_tiles_00")
        .m((bool (OIIO::ImageInput::*)(int, int, int, int, int, int, int, int, int, int, void *))
            &OIIO::ImageInput::read_native_tiles, "read_native_tiles_01")
        .m(&OIIO::ImageInput::send_to_input)
        .m(&OIIO::ImageInput::send_to_client)
        .m(&OIIO::ImageInput::set_ioproxy)
        .m(&OIIO::ImageInput::has_error)
        .m(&OIIO::ImageInput::geterror)
        /** TODO: instantiate this template
        .m(&OIIO::ImageInput::error)
        */
        /** TODO: instantiate this template
        .m(&OIIO::ImageInput::errorf)
        */
        /** TODO: instantiate this template
        .m(&OIIO::ImageInput::errorfmt)
        */
        /** TODO: instantiate this template
        .m(&OIIO::ImageInput::fmterror)
        */
        .m((void (OIIO::ImageInput::*)(int))
            &OIIO::ImageInput::threads, "threads_00")
        .m((int (OIIO::ImageInput::*)() const)
            &OIIO::ImageInput::threads, "threads_01")
        .m(&OIIO::ImageInput::lock)
        .m(&OIIO::ImageInput::unlock)
        .m(&OIIO::ImageInput::try_lock)
        .m(&OIIO::ImageInput::operator new)
        .m(&OIIO::ImageInput::operator delete)
    ;

    bbl::Class<OIIO::ImageOutput::unique_ptr>("ImageOutputPtr")
        .smartptr_to<OIIO::ImageOutput>()
    ;

    bbl::Class<OIIO::ImageOutput>()
        // .m((unique_ptr (*)(string_view, OIIO::Filesystem::IOProxy *, string_view))
        //     &OIIO::ImageOutput::create, "create_00")
        .m((OIIO::ImageOutput::unique_ptr (*)(const std::wstring &, OIIO::Filesystem::IOProxy *, const std::wstring &))
            &OIIO::ImageOutput::create, "create_01")
        .m((OIIO::ImageOutput::unique_ptr (*)(const std::string &, const std::string &))
            &OIIO::ImageOutput::create, "create_02")
        .m(&OIIO::ImageOutput::destroy)
        .m(&OIIO::ImageOutput::format_name)
        .m(&OIIO::ImageOutput::supports)
        .m((bool (OIIO::ImageOutput::*)(const std::string &, const OIIO::ImageSpec &, OIIO::ImageOutput::OpenMode))
            &OIIO::ImageOutput::open, "open_00")
        .m((bool (OIIO::ImageOutput::*)(const std::wstring &, const OIIO::ImageSpec &, OIIO::ImageOutput::OpenMode))
            &OIIO::ImageOutput::open, "open_01")
        .m((bool (OIIO::ImageOutput::*)(const std::string &, int, const OIIO::ImageSpec *))
            &OIIO::ImageOutput::open, "open_02")
        .m((bool (OIIO::ImageOutput::*)(const std::wstring &, int, const OIIO::ImageSpec *))
            &OIIO::ImageOutput::open, "open_03")
        .m(&OIIO::ImageOutput::spec)
        .m(&OIIO::ImageOutput::close)
        .m(&OIIO::ImageOutput::write_scanline)
        .m(&OIIO::ImageOutput::write_scanlines)
        .m(&OIIO::ImageOutput::write_tile)
        .m(&OIIO::ImageOutput::write_tiles)
        .m(&OIIO::ImageOutput::write_rectangle)
        .m(&OIIO::ImageOutput::write_image)
        .m(&OIIO::ImageOutput::write_deep_scanlines)
        .m(&OIIO::ImageOutput::write_deep_tiles)
        .m(&OIIO::ImageOutput::write_deep_image)
        .m(&OIIO::ImageOutput::set_thumbnail)
        .m(&OIIO::ImageOutput::copy_image)
        .m(&OIIO::ImageOutput::send_to_output)
        .m(&OIIO::ImageOutput::send_to_client)
        .m(&OIIO::ImageOutput::set_ioproxy)
        .m(&OIIO::ImageOutput::has_error)
        .m(&OIIO::ImageOutput::geterror)
        .m((void (OIIO::ImageOutput::*)(int))
            &OIIO::ImageOutput::threads, "setthreads")
        .m((int (OIIO::ImageOutput::*)() const)
            &OIIO::ImageOutput::threads, "getthreads")
    ;

    bbl::fn(&OIIO::roi_union);

    bbl::fn(&OIIO::roi_intersection);

    // bbl::fn((bool (*)(string_view, int))
    //         &OIIO::attribute, "attribute_01");

    // bbl::fn((bool (*)(string_view, float))
    //         &OIIO::attribute, "attribute_02");

    // bbl::fn((bool (*)(string_view, string_view))
    //         &OIIO::attribute, "attribute_03");

    // bbl::fn((bool (*)(string_view, int &))
    //         &OIIO::getattribute, "getattribute_01");

    // bbl::fn((bool (*)(string_view, float &))
    //         &OIIO::getattribute, "getattribute_02");

    // bbl::fn((bool (*)(string_view, std::string &))
    //         &OIIO::getattribute, "getattribute_03");

    // bbl::fn((bool (*)(string_view, char **))
    //         &OIIO::getattribute, "getattribute_04");

    // bbl::fn(&OIIO::get_int_attribute);

    // bbl::fn(&OIIO::get_float_attribute);

    // bbl::fn(&OIIO::get_string_attribute);

    // bbl::fn(&OIIO::get_extension_map);

    // bbl::fn(&OIIO::convert_types);

    // bbl::fn((bool (*)(int, int, int, int, const void *, OIIO::TypeDesc, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, void *, OIIO::TypeDesc, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, int, int))
    //         &OIIO::convert_image, "convert_image_01");

    // bbl::fn((bool (*)(int, int, int, int, const void *, OIIO::TypeDesc, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, void *, OIIO::TypeDesc, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, int, int, int))
    //         &OIIO::parallel_convert_image, "parallel_convert_image_01");

    /** TODO: instantiate this template
    bbl::fn((void (*)(const char *, const T1 &, const Args &...))
            &OIIO::debug, "debug_01");
    */

    /** TODO: instantiate this template
    bbl::fn(&OIIO::debugfmt);
    */

    /** TODO: instantiate this template
    bbl::fn(&OIIO::fmtdebug);
    */

    /** TODO: instantiate this template
    bbl::fn(&OIIO::debugf);
    */


}
