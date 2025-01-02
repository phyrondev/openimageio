#include <babble>

#include <OpenImageIO/texture.h>

namespace bblext {

OIIO::TextureSystem::TextureHandle *
TextureSystem_texture_handle(std::shared_ptr<OIIO::TextureSystem> self,
                             OIIO::ustring file_name,
                             OIIO::TextureSystem::Perthread *per_thread) {
  return self->get_texture_handle(file_name, per_thread);
}

bool TextureSystem_texture(std::shared_ptr<OIIO::TextureSystem> self,
                           OIIO::TextureSystem::TextureHandle *texture_handle,
                           OIIO::TextureSystem::Perthread *per_thread,
                           OIIO::TextureOpt &options, float s, float t,
                           float ds_dx, float dt_dx, float ds_dy, float dt_dy,
                           int channel_count, float *result, float *d_result_ds,
                           float *d_result_dt) {
  return self->texture(texture_handle, per_thread, options, s, t, ds_dx, dt_dx,
                       ds_dy, dt_dy, channel_count, result, d_result_ds,
                       d_result_dt);
}

bool TextureSystem_texture_multi(
    std::shared_ptr<OIIO::TextureSystem> self,
    OIIO::TextureSystem::TextureHandle *texture_handle,
    OIIO::TextureSystem::Perthread *per_thread, OIIO::TextureOptBatch &options,
    OIIO::Tex::RunMask mask, const float *s, const float *t, const float *ds_dx,
    const float *dt_dx, const float *ds_dy, const float *dt_dy,
    int channel_count, float *result, float *d_result_ds, float *d_result_dt) {
  return self->texture(texture_handle, per_thread, options, mask, s, t, ds_dx,
                       dt_dx, ds_dy, dt_dy, channel_count, result, d_result_ds,
                       d_result_dt);
}

void TextureSystem_make_texture_options(
    int first_channel, int sub_image, const char *sub_image_name,
    OIIO::Tex::Wrap s_wrap, OIIO::Tex::Wrap t_wrap, OIIO::Tex::MipMode mip_mode,
    OIIO::Tex::InterpMode interpolation_mode, int anisotropic_samples,
    bool conservative_filter, float s_blur, float t_blur, float s_width,
    float t_width, float fill, float *missing_color, float random,
    OIIO::Tex::Wrap r_wrap, float r_blur, float r_width,
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

void TextureSystem_make_texture_batch_options(
    int first_channel, int sub_image, const char *sub_image_name,
    OIIO::Tex::Wrap s_wrap, OIIO::Tex::Wrap t_wrap, OIIO::Tex::MipMode mip_mode,
    OIIO::Tex::InterpMode interpolation_mode, int anisotropic_samples,
    bool conservative_filter, float *s_blur, float *t_blur, float *s_width,
    float *t_width, float fill, float *missing_color, float *random,
    OIIO::Tex::Wrap r_wrap, float *r_blur, float *r_width,
    OIIO::TextureOptBatch *dest) {
  // Safe but slower. We initialize the struct with defaults and then overwrite
  // it.
  dest = new OIIO::TextureOptBatch();

  dest->firstchannel = first_channel;
  dest->subimage = sub_image;
  dest->subimagename = OIIO::ustring(sub_image_name);
  dest->swrap = int(s_wrap);
  dest->twrap = int(t_wrap);
  dest->mipmode = int(mip_mode);
  dest->interpmode = int(interpolation_mode);
  dest->anisotropic = anisotropic_samples;
  dest->conservative_filter = conservative_filter;
  memcpy(dest->sblur, s_blur, sizeof(float) * OIIO::Tex::BatchWidth);
  memcpy(dest->tblur, s_blur, sizeof(float) * OIIO::Tex::BatchWidth);
  memcpy(dest->swidth, s_blur, sizeof(float) * OIIO::Tex::BatchWidth);
  memcpy(dest->twidth, s_blur, sizeof(float) * OIIO::Tex::BatchWidth);
  dest->fill = fill;
  dest->missingcolor = missing_color;
  memcpy(dest->rnd, random, sizeof(float) * OIIO::Tex::BatchWidth);
  dest->rwrap = int(r_wrap);
  memcpy(dest->rblur, r_blur, sizeof(float) * OIIO::Tex::BatchWidth);
  memcpy(dest->rwidth, r_width, sizeof(float) * OIIO::Tex::BatchWidth);
}

} // namespace bblext

BBL_MODULE(oiio) {

  bbl::Class<std::shared_ptr<OIIO::TextureSystem>>("TextureSystemSharedPtr")
      .smartptr_to<OIIO::TextureSystem>()
      .ignore_all_unbound();

  bbl::Enum<OIIO::TextureOpt::Wrap>();
  bbl::Enum<OIIO::TextureOpt::MipMode>();
  bbl::Enum<OIIO::TextureOpt::InterpMode>();

  // bbl::constant<OIIO::BatchWidth>();

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
  bbl::Class<OIIO::TextureOptBatch>().ctor(
      bbl::Class<OIIO::TextureOptBatch>::Ctor<>(), "default");
  bbl::ClassIncomplete<OIIO::TextureSystem::TextureHandle>();

  // bbl::Class<OIIO::TextureOpt>()
  //   .value_type()

  bbl::fn(&bblext::TextureSystem_texture_handle,
          "TextureSystem_texture_handle");
  bbl::fn(&bblext::TextureSystem_texture, "TextureSystem_texture");
  bbl::fn(&bblext::TextureSystem_texture_multi, "TextureSystem_texture_multi");
  bbl::fn(&bblext::TextureSystem_make_texture_options,
          "TextureSystem_make_texture_options");
  bbl::fn(&bblext::TextureSystem_make_texture_batch_options,
          "TextureSystem_make_texture_batch_options");
}
