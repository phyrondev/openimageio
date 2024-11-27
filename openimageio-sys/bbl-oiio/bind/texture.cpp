#include <babble>

#include <OpenImageIO/texture.h>

namespace bblext {

OIIO::TextureSystem::TextureHandle *
TextureSystem_texture_handle(std::shared_ptr<OIIO::TextureSystem> self,
                             OIIO::ustring file_name,
                             OIIO::TextureSystem::Perthread *per_thread) {
  return self->get_texture_handle(file_name, per_thread);
}

void TextureSystem_texture(std::shared_ptr<OIIO::TextureSystem> self,
                           OIIO::TextureSystem::TextureHandle *texture_handle,
                           OIIO::TextureSystem::Perthread *per_thread,
                           OIIO::TextureOpt &options, float s, float t,
                           float ds_dx, float dt_dx, float ds_dy, float dt_dy,
                           int channel_count, float *result, float *d_result_ds,
                           float *d_result_dt) {
  self->texture(texture_handle, per_thread, options, s, t, ds_dx, dt_dx, ds_dy,
                dt_dy, channel_count, result, d_result_ds, d_result_dt);
}

void TextureSystem_make_texture_options(
    int first_channel, int sub_image, const char *sub_image_name,
    OIIO::TextureOpt::Wrap s_wrap, OIIO::TextureOpt::Wrap t_wrap,
    OIIO::TextureOpt::MipMode mip_mode,
    OIIO::TextureOpt::InterpMode interpolation_mode, int anisotropic_samples,
    bool conservative_filter, float s_blur, float t_blur, float s_width,
    float t_width, float fill, float *missing_color, float random,
    OIIO::TextureOpt::Wrap r_wrap, float r_blur, float r_width,
    OIIO::TextureOpt *dest) {
  // Safe but slower. We initialize the struct with defaults and then overwrite
  // it.
  dest = new OIIO::TextureOpt();

  dest->firstchannel = first_channel;
  dest->subimage = sub_image;
  dest->subimagename = OIIO::ustring(sub_image_name);
  dest->swrap = s_wrap;
  dest->twrap = t_wrap;
  dest->mipmode = mip_mode;
  dest->interpmode = interpolation_mode;
  dest->anisotropic = anisotropic_samples;
  dest->conservative_filter = conservative_filter;
  dest->sblur = s_blur;
  dest->tblur = t_blur;
  dest->swidth = s_width;
  dest->twidth = t_width;
  dest->fill = fill;
  dest->missingcolor = missing_color;
  dest->rnd = random;
  dest->rwrap = r_wrap;
  dest->rblur = r_blur;
  dest->rwidth = r_width;
}

} // namespace bblext

BBL_MODULE(oiio) {

  bbl::Class<std::shared_ptr<OIIO::TextureSystem>>("TextureSystemSharedPtr")
      .smartptr_to<OIIO::TextureSystem>()
      .ignore_all_unbound();

  bbl::Enum<OIIO::TextureOpt::Wrap>();
  bbl::Enum<OIIO::TextureOpt::MipMode>();
  bbl::Enum<OIIO::TextureOpt::InterpMode>();

  bbl::Class<OIIO::TextureSystem>()
      .m((std::shared_ptr<OIIO::TextureSystem>(*)(bool, OIIO::ImageCache *)) &
             OIIO::TextureSystem::create,
         "create")
      .m((void (*)(std::shared_ptr<OIIO::TextureSystem>,
                   bool))&OIIO::TextureSystem::destroy,
         "destroy");

  bbl::ClassIncomplete<OIIO::TextureSystem::Perthread>();
  bbl::Class<OIIO::TextureOpt>().ctor(bbl::Class<OIIO::TextureOpt>::Ctor<>(),
                                      "default");
  bbl::ClassIncomplete<OIIO::TextureSystem::TextureHandle>();

  // bbl::Class<OIIO::TextureOpt>()
  //   .value_type()

  bbl::fn(&bblext::TextureSystem_texture_handle,
          "TextureSystem_texture_handle");
  bbl::fn(&bblext::TextureSystem_texture, "TextureSystem_texture");
  bbl::fn(&bblext::TextureSystem_make_texture_options,
          "TextureSystem_make_texture_options");
}
