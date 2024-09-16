#include <babble>

#include <OpenImageIO/texture.h>

namespace bblext {

    OIIO::TextureSystem::TextureHandle* TextureSystem_texture_handle(
        OIIO::TextureSystem* self,
        OIIO::ustring file_name,
        OIIO::TextureSystem::Perthread* per_thread
    ) {
        return self->get_texture_handle(file_name, per_thread);
    }

    void TextureSystem_texture(
        OIIO::TextureSystem* self,
        OIIO::TextureSystem::TextureHandle* texture_handle,
        OIIO::TextureSystem::Perthread* per_thread,
        OIIO::TextureOpt &options,
        float s, float t,
        float ds_dx, float dt_dx,
        float ds_dy, float dt_dy,
        int channel_count, float *result,
        float *d_result_ds, float *d_result_dt
    ) {
        self->texture(
            texture_handle,
            per_thread,
            options,
            s, t,
            ds_dx, dt_dx,
            ds_dy, dt_dy,
            channel_count,
            result,
            d_result_ds, d_result_dt
        );
    }

    void TextureSystem_make_texture_options(
        int first_channel,
        int sub_image,
        const char* sub_image_name,
        OIIO::TextureOpt::Wrap s_wrap,
        OIIO::TextureOpt::Wrap t_wrap,
        OIIO::TextureOpt::MipMode mip_mode,
        OIIO::TextureOpt::InterpMode interpolation_mode,
        int anisotropic_samples,
        bool conservative_filter,
        float s_blur,
        float t_blur,
        float s_width,
        float t_width,
        float fill,
        float* missing_color,
        float time,
        float random,
        int samples,
        OIIO::TextureOpt::Wrap r_wrap,
        float r_blur,
        float r_width,
        OIIO::TextureOpt *dest
    ){
        // Safe but slower. We initialize the struct with defaults and then overwrite it.
        OIIO::TextureOpt t_o = OIIO::TextureOpt();

        t_o.firstchannel = first_channel;
        t_o.subimage = sub_image;
        t_o.subimagename = OIIO::ustring(sub_image_name);
        t_o.swrap = s_wrap;
        t_o.twrap = t_wrap;
        t_o.mipmode = mip_mode;
        t_o.interpmode = interpolation_mode;
        t_o.anisotropic = anisotropic_samples;
        t_o.conservative_filter = conservative_filter;
        t_o.sblur = s_blur;
        t_o.tblur = t_blur;
        t_o.swidth = s_width;
        t_o.twidth = t_width;
        t_o.fill = fill;
        t_o.missingcolor = missing_color;
        t_o.time = time;
        t_o.rnd = random;
        t_o.samples = samples;
        t_o.rwrap = r_wrap;
        t_o.rblur = r_blur;
        t_o.rwidth = r_width;

        *dest = t_o;
    }

}

BBL_MODULE(oiio) {

    bbl::Enum<OIIO::TextureOpt::Wrap>();
    bbl::Enum<OIIO::TextureOpt::MipMode>();
    bbl::Enum<OIIO::TextureOpt::InterpMode>();

    bbl::Class<OIIO::TextureSystem>()
        .m((OIIO::TextureSystem* (*)(bool, OIIO::ImageCache *))
            &OIIO::TextureSystem::create, "create")
        .m((void (*)(OIIO::TextureSystem*, bool))
            &OIIO::TextureSystem::destroy, "destroy")
    ;

    bbl::ClassIncomplete<OIIO::TextureSystem::Perthread>();
    bbl::Class<OIIO::TextureOpt>()
        .ctor(bbl::Class<OIIO::TextureOpt>::Ctor<>(), "default")
    ;
    bbl::ClassIncomplete<OIIO::TextureSystem::TextureHandle>();

   // bbl::Class<OIIO::TextureOpt>()
     //   .value_type()

    bbl::fn(&bblext::TextureSystem_texture_handle, "TextureSystem_texture_handle");
    bbl::fn(&bblext::TextureSystem_texture, "TextureSystem_texture");
    bbl::fn(&bblext::TextureSystem_make_texture_options, "TextureSystem_make_texture_options");
}
