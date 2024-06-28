#include <babble>

#include <OpenImageIO/imagebuf.h>
#include <OpenImageIO/imagecache.h>
#include <OpenImageIO/deepdata.h>

namespace bblext {

auto DeepData_channelname(OIIO::DeepData const& dd, int c) -> char const* {
    return dd.channelname(c).c_str();
}

auto DeepData_set_all_samples(OIIO::DeepData& dd, unsigned int const* samples, long long num) -> void {
    dd.set_all_samples({samples, num});
}

auto DeepData_all_samples(OIIO::DeepData& dd, unsigned int const** samples, long long* num) -> void {
    auto result = dd.all_samples();
    *samples = result.data();
    *num = result.size();
}

auto DeepData_all_channeltypes(OIIO::DeepData& dd, OIIO::TypeDesc const** samples, long long* num) -> void {
    auto result = dd.all_channeltypes();
    *samples = result.data();
    *num = result.size();
}

auto DeepData_all_data(OIIO::DeepData& dd, char const** samples, long long* num) -> void {
    auto result = dd.all_data();
    *samples = result.data();
    *num = result.size();
}


}

BBL_MODULE(oiio) {

    bbl::Class<OIIO::DeepData>()
        .ctor(bbl::Class<OIIO::DeepData>::Ctor<>(), "default")
        .ctor(bbl::Class<OIIO::DeepData>::Ctor<const OIIO::ImageSpec &>("spec"), "ctor_01")
        /// XXX: ctor wrappers
        // .ctor(bbl::Class<OIIO::DeepData>::Ctor<const OIIO::DeepData &, OIIO::cspan<OIIO::TypeDesc>>("src", "channeltypes"), "copy_with_new_channel_types")
        .m(&OIIO::DeepData::operator=, "op_assign")
        .m(&OIIO::DeepData::clear)
        .m(&OIIO::DeepData::free)
        // .m(bbl::Wrap((void (OIIO::DeepData::*)(int64_t, int, OIIO::cspan<OIIO::TypeDesc>, OIIO::cspan<std::string>)) &OIIO::DeepData::init, 
        //             [](OIIO::DeepData& _this, int64_t npix, int nchans, OIIO::TypeDesc const* types, long long num_types, std::string const* names, long long num_names) -> void {
        //         _this.init(npix, nchans, OIIO::cspan<OIIO::TypeDesc>(types, num_types), OIIO::cspan<std::string>(names, num_names));
        //     }), "init")
        .m((void (OIIO::DeepData::*)(const OIIO::ImageSpec &))
            &OIIO::DeepData::init, "init_with_imagespec")
        .m(&OIIO::DeepData::initialized)
        .m(&OIIO::DeepData::allocated)
        .m(&OIIO::DeepData::pixels)
        .m(&OIIO::DeepData::channels)
        .m(&OIIO::DeepData::Z_channel)
        .m(&OIIO::DeepData::Zback_channel)
        .m(&OIIO::DeepData::A_channel)
        .m(&OIIO::DeepData::AR_channel)
        .m(&OIIO::DeepData::AG_channel)
        .m(&OIIO::DeepData::AB_channel)
        // .m(&OIIO::DeepData::channelname)
        .m(&OIIO::DeepData::channeltype)
        .m(&OIIO::DeepData::channelsize)
        .m(&OIIO::DeepData::samplesize)
        .m(&OIIO::DeepData::same_channeltypes)
        .m(&OIIO::DeepData::samples)
        .m(&OIIO::DeepData::set_samples)
        // .m(&OIIO::DeepData::set_all_samples)
        .m(&OIIO::DeepData::set_capacity)
        .m(&OIIO::DeepData::capacity)
        .m(&OIIO::DeepData::insert_samples)
        .m(&OIIO::DeepData::erase_samples)
        .m(&OIIO::DeepData::deep_value)
        .m(&OIIO::DeepData::deep_value_uint)
        .m((void (OIIO::DeepData::*)(int64_t, int, int, float))
            &OIIO::DeepData::set_deep_value, "set_deep_value_00")
        .m((void (OIIO::DeepData::*)(int64_t, int, int, uint32_t))
            &OIIO::DeepData::set_deep_value, "set_deep_value_01")
        .m((void * (OIIO::DeepData::*)(int64_t, int, int))
            &OIIO::DeepData::data_ptr, "data_ptr_00")
        .m((const void * (OIIO::DeepData::*)(int64_t, int, int) const)
            &OIIO::DeepData::data_ptr, "data_ptr_01")
        .m(bbl::Wrap(
            &OIIO::DeepData::all_channeltypes, 
            [](OIIO::DeepData const& _this, OIIO::TypeDesc const** types, long long* num_types) -> void {
                auto span = _this.all_channeltypes();
                *types = span.data();
                *num_types = span.size();
            }
            ))
        .m(bbl::Wrap(
            &OIIO::DeepData::all_samples, 
            [](OIIO::DeepData const& _this, unsigned const** samples, long long* num_types) -> void {
                auto span = _this.all_samples();
                *samples = span.data();
                *num_types = span.size();
            }
            ))
        .m(bbl::Wrap(
            &OIIO::DeepData::all_data, 
            [](OIIO::DeepData const& _this, char const** data, long long* num_types) -> void {
                auto span = _this.all_data();
                *data = span.data();
                *num_types = span.size();
            }
            ))
        .m(&OIIO::DeepData::get_pointers)
        .m(&OIIO::DeepData::copy_deep_sample)
        .m(&OIIO::DeepData::copy_deep_pixel)
        .m(&OIIO::DeepData::split)
        .m(&OIIO::DeepData::sort)
        .m(&OIIO::DeepData::merge_overlaps)
        .m(&OIIO::DeepData::merge_deep_pixels)
        .m(&OIIO::DeepData::opaque_z)
        .m(&OIIO::DeepData::occlusion_cull)
    ;

    bbl::Class<std::vector<void*>>("PointerVector");

    bbl::fn(&bblext::DeepData_channelname);
    bbl::fn(&bblext::DeepData_set_all_samples);
}
