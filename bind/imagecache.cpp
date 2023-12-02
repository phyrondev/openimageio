#include <babble>

#include <OpenImageIO/imagecache.h>

BBL_MODULE(oiio) {

    bbl::Class<OIIO::ImageCache>()
        .m(&OIIO::ImageCache::create)
        .m(&OIIO::ImageCache::destroy)
        .m(bbl::Wrap(
            (bool (OIIO::ImageCache::*)(OIIO::string_view, OIIO::TypeDesc, const void *))
            &OIIO::ImageCache::attribute,
            [](OIIO::ImageCache& _this, char const* name, OIIO::TypeDesc type, void const* data) -> bool {
                return _this.attribute(name, type, data);
            }), "attribute")
        .m(bbl::Wrap((bool (OIIO::ImageCache::*)(OIIO::string_view, OIIO::TypeDesc, void *) const)
            &OIIO::ImageCache::getattribute,
            [](OIIO::ImageCache const& _this, char const* name, OIIO::TypeDesc type, void* data) -> bool {
                return _this.getattribute(name, type, data);
            }
        ), "getattribute")
        
        .m(&OIIO::ImageCache::get_perthread_info)
        .m(&OIIO::ImageCache::create_thread_info)
        .m(&OIIO::ImageCache::destroy_thread_info)
        .m((OIIO::ImageCache::ImageHandle * (OIIO::ImageCache::*)(OIIO::ustring, OIIO::ImageCache::Perthread *))
            &OIIO::ImageCache::get_image_handle, "get_image_handle")
        // .m((OIIO::ImageCache::ImageHandle * (OIIO::ImageCache::*)(const std::wstring &, OIIO::ImageCache::Perthread *))
        //     &OIIO::ImageCache::get_image_handle, "get_image_handle_w")
        .m(&OIIO::ImageCache::good)
        .m(&OIIO::ImageCache::filename_from_handle)
        .m(&OIIO::ImageCache::resolve_filename)
        .m((bool (OIIO::ImageCache::*)(OIIO::ustring, int, int, OIIO::ustring, OIIO::TypeDesc, void *))
            &OIIO::ImageCache::get_image_info, "get_image_info")
        .m((bool (OIIO::ImageCache::*)(OIIO::ImageCache::ImageHandle *, OIIO::ImageCache::Perthread *, int, int, OIIO::ustring, OIIO::TypeDesc, void *))
            &OIIO::ImageCache::get_image_info, "get_image_info_from_handle")
        .m((bool (OIIO::ImageCache::*)(OIIO::ustring, OIIO::ImageSpec &, int, int, bool))
            &OIIO::ImageCache::get_imagespec, "get_imagespec")
        .m((bool (OIIO::ImageCache::*)(OIIO::ImageCache::ImageHandle *, OIIO::ImageCache::Perthread *, OIIO::ImageSpec &, int, int, bool))
            &OIIO::ImageCache::get_imagespec, "get_imagespec_from_handle")
        .m((const OIIO::ImageSpec * (OIIO::ImageCache::*)(OIIO::ustring, int, int, bool))
            &OIIO::ImageCache::imagespec, "imagespec")
        .m((const OIIO::ImageSpec * (OIIO::ImageCache::*)(OIIO::ImageCache::ImageHandle *, OIIO::ImageCache::Perthread *, int, int, bool))
            &OIIO::ImageCache::imagespec, "imagespec_from_handle")
        .m((bool (OIIO::ImageCache::*)(OIIO::ustring, OIIO::ImageBuf &, int))
            &OIIO::ImageCache::get_thumbnail, "get_thumbnail")
        .m((bool (OIIO::ImageCache::*)(OIIO::ImageCache::ImageHandle *, OIIO::ImageCache::Perthread *, OIIO::ImageBuf &, int))
            &OIIO::ImageCache::get_thumbnail, "get_thumbnail_from_handle")
        .m((bool (OIIO::ImageCache::*)(OIIO::ustring, int, int, int, int, int, int, int, int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, int, int))
            &OIIO::ImageCache::get_pixels, "get_pixels_00")
        .m((bool (OIIO::ImageCache::*)(OIIO::ImageCache::ImageHandle *, OIIO::ImageCache::Perthread *, int, int, int, int, int, int, int, int, int, int, OIIO::TypeDesc, void *, OIIO::stride_t, OIIO::stride_t, OIIO::stride_t, int, int))
            &OIIO::ImageCache::get_pixels, "get_pixels_01")
        .m((bool (OIIO::ImageCache::*)(OIIO::ustring, int, int, int, int, int, int, int, int, OIIO::TypeDesc, void *))
            &OIIO::ImageCache::get_pixels, "get_pixels_02")
        .m((bool (OIIO::ImageCache::*)(OIIO::ImageCache::ImageHandle *, OIIO::ImageCache::Perthread *, int, int, int, int, int, int, int, int, OIIO::TypeDesc, void *))
            &OIIO::ImageCache::get_pixels, "get_pixels_03")
        .m((void (OIIO::ImageCache::*)(OIIO::ustring, bool))
            &OIIO::ImageCache::invalidate, "invalidate_00")
        .m((void (OIIO::ImageCache::*)(OIIO::ImageCache::ImageHandle *, bool))
            &OIIO::ImageCache::invalidate, "invalidate_01")
        .m(&OIIO::ImageCache::invalidate_all)
        .m(&OIIO::ImageCache::close)
        .m(&OIIO::ImageCache::close_all)
        .m((OIIO::ImageCache::Tile * (OIIO::ImageCache::*)(OIIO::ustring, int, int, int, int, int, int, int))
            &OIIO::ImageCache::get_tile, "get_tile")
        .m((OIIO::ImageCache::Tile * (OIIO::ImageCache::*)(OIIO::ImageCache::ImageHandle *, OIIO::ImageCache::Perthread *, int, int, int, int, int, int, int))
            &OIIO::ImageCache::get_tile, "get_tile_from_handle")
        .m(&OIIO::ImageCache::release_tile)
        .m(&OIIO::ImageCache::tile_format)
        .m(&OIIO::ImageCache::tile_roi)
        .m(&OIIO::ImageCache::tile_pixels)
        // Callback 
        // .m(&OIIO::ImageCache::add_file)
        .m(&OIIO::ImageCache::add_tile)
        .m(&OIIO::ImageCache::has_error)
        .m(&OIIO::ImageCache::geterror)
        .m(&OIIO::ImageCache::getstats)
        .m(&OIIO::ImageCache::reset_stats)
        // .m(&OIIO::ImageCache::operator=, "op_assign")
    ;

    
    bbl::ClassIncomplete<OIIO::ImageCache::Perthread>();
    bbl::ClassIncomplete<OIIO::ImageCache::ImageHandle>();
    bbl::ClassIncomplete<OIIO::ImageCache::Tile>();

}
