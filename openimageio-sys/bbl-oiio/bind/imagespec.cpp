#include <babble>

#include <OpenImageIO/imagebuf.h>

// bindfile
namespace bblext {
int ImageSpec_get_x(OIIO::ImageSpec const &self) { return self.x; }

void ImageSpec_set_x(OIIO::ImageSpec &self, int value) { self.x = value; }

int ImageSpec_get_y(OIIO::ImageSpec const &self) { return self.y; }

void ImageSpec_set_y(OIIO::ImageSpec &self, int value) { self.y = value; }

int ImageSpec_get_z(OIIO::ImageSpec const &self) { return self.z; }

void ImageSpec_set_z(OIIO::ImageSpec &self, int value) { self.z = value; }

int ImageSpec_get_width(OIIO::ImageSpec const &self) { return self.width; }

void ImageSpec_set_width(OIIO::ImageSpec &self, int value) {
  self.width = value;
}

int ImageSpec_get_height(OIIO::ImageSpec const &self) { return self.height; }

void ImageSpec_set_height(OIIO::ImageSpec &self, int value) {
  self.height = value;
}

int ImageSpec_get_depth(OIIO::ImageSpec const &self) { return self.depth; }

void ImageSpec_set_depth(OIIO::ImageSpec &self, int value) {
  self.depth = value;
}

int ImageSpec_get_full_x(OIIO::ImageSpec const &self) { return self.full_x; }

void ImageSpec_set_full_x(OIIO::ImageSpec &self, int value) {
  self.full_x = value;
}

int ImageSpec_get_full_y(OIIO::ImageSpec const &self) { return self.full_y; }

void ImageSpec_set_full_y(OIIO::ImageSpec &self, int value) {
  self.full_y = value;
}

int ImageSpec_get_full_z(OIIO::ImageSpec const &self) { return self.full_z; }

void ImageSpec_set_full_z(OIIO::ImageSpec &self, int value) {
  self.full_z = value;
}

int ImageSpec_get_full_width(OIIO::ImageSpec const &self) {
  return self.full_width;
}

void ImageSpec_set_full_width(OIIO::ImageSpec &self, int value) {
  self.full_width = value;
}

int ImageSpec_get_full_height(OIIO::ImageSpec const &self) {
  return self.full_height;
}

void ImageSpec_set_full_height(OIIO::ImageSpec &self, int value) {
  self.full_height = value;
}

int ImageSpec_get_full_depth(OIIO::ImageSpec const &self) {
  return self.full_depth;
}

void ImageSpec_set_full_depth(OIIO::ImageSpec &self, int value) {
  self.full_depth = value;
}

int ImageSpec_get_tile_width(OIIO::ImageSpec const &self) {
  return self.tile_width;
}

void ImageSpec_set_tile_width(OIIO::ImageSpec &self, int value) {
  self.tile_width = value;
}

int ImageSpec_get_tile_height(OIIO::ImageSpec const &self) {
  return self.tile_height;
}

void ImageSpec_set_tile_height(OIIO::ImageSpec &self, int value) {
  self.tile_height = value;
}

int ImageSpec_get_tile_depth(OIIO::ImageSpec const &self) {
  return self.tile_depth;
}

void ImageSpec_set_tile_depth(OIIO::ImageSpec &self, int value) {
  self.tile_depth = value;
}

int ImageSpec_get_nchannels(OIIO::ImageSpec const &self) {
  return self.nchannels;
}

void ImageSpec_set_nchannels(OIIO::ImageSpec &self, int value) {
  self.nchannels = value;
}

unsigned char ImageSpec_get_format_basetype(OIIO::ImageSpec const &self) {
  return self.format.basetype;
}

void ImageSpec_set_format_basetype(OIIO::ImageSpec &self, unsigned char value) {
  self.format.basetype = value;
}

std::vector<OIIO::TypeDesc> const &
ImageSpec_get_channelformats_ref(OIIO::ImageSpec const &self) {
  return self.channelformats;
}

void ImageSpec_set_channelformats(OIIO::ImageSpec &self,
                                  std::vector<OIIO::TypeDesc> value) {
  self.channelformats = value;
}

int ImageSpec_get_alpha_channel(OIIO::ImageSpec const &self) {
  return self.alpha_channel;
}

void ImageSpec_set_alpha_channel(OIIO::ImageSpec &self, int value) {
  self.alpha_channel = value;
}

int ImageSpec_get_z_channel(OIIO::ImageSpec const &self) {

  return self.z_channel;
}

void ImageSpec_set_z_channel(OIIO::ImageSpec &self, int value) {
  self.z_channel = value;
}

bool ImageSpec_get_deep(OIIO::ImageSpec const &self) { return self.deep; }

void ImageSpec_set_deep(OIIO::ImageSpec &self, bool value) {
  self.deep = value;
}

void ImageSpec_clear_and_reserve_channelformats(OIIO::ImageSpec &self,
                                                size_t size) {
  self.channelformats.clear();
  self.channelformats.reserve(size);
}

void ImageSpec_push_channelformat(OIIO::ImageSpec &self, OIIO::TypeDesc value) {
  self.channelformats.push_back(value);
  self.nchannels = self.channelformats.size();
}

void ImageSpec_clear_and_reserve_channelnames(OIIO::ImageSpec &self,
                                              size_t size) {
  self.channelnames.clear();
  self.channelnames.reserve(size);
}

void ImageSpec_push_channelname(OIIO::ImageSpec &self,
                                std::string const &value) {
  self.channelnames.push_back(value);
}

std::vector<std::string> const &
ImageSpec_get_channelnames(OIIO::ImageSpec const &self) {
  return self.channelnames;
}
} // namespace bblext

BBL_MODULE(oiio) {

  bbl::Class<OIIO::ImageSpec>()
      //.value_type()
      .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<OIIO::TypeDesc>("format"), "new")
      // .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<OIIO::string_view>("format"),
      // "with_named_type")
      .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<int, int, int, OIIO::TypeDesc>(
                "xres", "yres", "nchans", "fmt"),
            "with_dimensions")
      // .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<int, int, int,
      // string_view>("xres", "yres", "nchans", "fmt"), "ctor_03")
      .ctor(
          bbl::Class<OIIO::ImageSpec>::Ctor<const OIIO::ROI &, OIIO::TypeDesc>(
              "roi", "fmt"),
          "with_region")
      // .ctor(bbl::Class<OIIO::ImageSpec>::Ctor<const OIIO::ROI &,
      // string_view>("roi", "fmt"), "ctor_05")
      .m((void(OIIO::ImageSpec::*)(OIIO::TypeDesc)) &
             OIIO::ImageSpec::set_format,
         "set_format")
      .m(bbl::Wrap((void(OIIO::ImageSpec::*)(OIIO::string_view)) &
                       OIIO::ImageSpec::set_format,
                   [](OIIO::ImageSpec &_this, char const *fmt) -> void {
                     _this.set_format(fmt);
                   }),
         "set_format_with_typename")
      .m(&OIIO::ImageSpec::default_channel_names)
      .m((size_t(OIIO::ImageSpec::*)() const)&OIIO::ImageSpec::channel_bytes,
         "channel_bytes_00")
      .m((size_t(OIIO::ImageSpec::*)(int, bool)
              const)&OIIO::ImageSpec::channel_bytes,
         "channel_bytes_01")
      .m((size_t(OIIO::ImageSpec::*)(bool) const)&OIIO::ImageSpec::pixel_bytes,
         "pixel_bytes_00")
      .m((size_t(OIIO::ImageSpec::*)(int, int, bool)
              const)&OIIO::ImageSpec::pixel_bytes,
         "pixel_bytes_01")
      .m(&OIIO::ImageSpec::scanline_bytes)
      .m(&OIIO::ImageSpec::tile_pixels)
      .m(&OIIO::ImageSpec::tile_bytes)
      .m(&OIIO::ImageSpec::image_pixels)
      .m(&OIIO::ImageSpec::image_bytes)
      .m(&OIIO::ImageSpec::size_t_safe)
      .m((void (*)(OIIO::stride_t &, OIIO::stride_t &, OIIO::stride_t &,
                   OIIO::stride_t, int, int, int))&OIIO::ImageSpec::auto_stride,
         "auto_stride_00")
      .m((void (*)(OIIO::stride_t &, OIIO::stride_t &, OIIO::stride_t &,
                   OIIO::TypeDesc, int, int, int))&OIIO::ImageSpec::auto_stride,
         "auto_stride_01")
      .m((void (*)(OIIO::stride_t &, OIIO::TypeDesc,
                   int))&OIIO::ImageSpec::auto_stride,
         "auto_stride_02")
      .m(bbl::Wrap((void(OIIO::ImageSpec::*)(OIIO::string_view, OIIO::TypeDesc,
                                             const void *)) &
                       OIIO::ImageSpec::attribute,
                   [](OIIO::ImageSpec &_this, char const *name,
                      OIIO::TypeDesc type, void const *data) -> void {
                     _this.attribute(name, type, data);
                   }),
         "attribute")
      .m(bbl::Wrap(&OIIO::ImageSpec::erase_attribute,
                   [](OIIO::ImageSpec &_this, char const *name,
                      OIIO::TypeDesc search_type, bool casesensitive) -> void {
                     _this.erase_attribute(name, search_type, casesensitive);
                   }))
      .m(bbl::Wrap(
             (OIIO::ParamValue *
              (OIIO::ImageSpec::*)(OIIO::string_view, OIIO::TypeDesc, bool)) &
                 OIIO::ImageSpec::find_attribute,
             [](OIIO::ImageSpec &_this, char const *name, OIIO::TypeDesc type,
                bool casesensitive) -> OIIO::ParamValue * {
               return _this.find_attribute(name, type, casesensitive);
             }),
         "find_attribute")
      .m(bbl::Wrap((OIIO::ParamValue const *(
                       OIIO::ImageSpec::*)(OIIO::string_view, OIIO::TypeDesc,
                                           bool) const) &
                       OIIO::ImageSpec::find_attribute,
                   [](OIIO::ImageSpec const &_this, char const *name,
                      OIIO::TypeDesc type,
                      bool casesensitive) -> OIIO::ParamValue const * {
                     return _this.find_attribute(name, type, casesensitive);
                   }),
         "find_attribute_const")
      .m(bbl::Wrap(&OIIO::ImageSpec::getattributetype,
                   [](OIIO::ImageSpec const &_this, char const *name,
                      bool casesensitive) -> OIIO::TypeDesc {
                     return _this.getattributetype(name, casesensitive);
                   }))
      .m(bbl::Wrap(
          &OIIO::ImageSpec::getattribute,
          [](OIIO::ImageSpec const &_this, char const *name,
             OIIO::TypeDesc type, void *value, bool casesensitive) -> bool {
            return _this.getattribute(name, type, value, casesensitive);
          }))
      .m(&OIIO::ImageSpec::metadata_val)
      .m(&OIIO::ImageSpec::serialize)
      .m(&OIIO::ImageSpec::to_xml)
      .m(&OIIO::ImageSpec::from_xml)
      .m(bbl::Wrap(
          &OIIO::ImageSpec::decode_compression_metadata,
          [](OIIO::ImageSpec const &_this, char const *name, int defaultqual,
             char const **compression, long long *compression_len,
             int *compression_quality) -> void {
            auto result = _this.decode_compression_metadata(name, defaultqual);
            *compression = OIIO::c_str(result.first);
            *compression_len = result.first.size();
            *compression_quality = result.second;
          }))
      .m(&OIIO::ImageSpec::valid_tile_range)
      .m(&OIIO::ImageSpec::channelformat)
      .m(bbl::Wrap(&OIIO::ImageSpec::channel_name,
                   [](OIIO::ImageSpec const &_this, int chan, char const **name,
                      long long *len) -> void {
                     auto sv = _this.channel_name(chan);
                     *name = OIIO::c_str(sv);
                     *len = sv.size();
                   }))
      .m(&OIIO::ImageSpec::get_channelformats)
      .m(bbl::Wrap(&OIIO::ImageSpec::channelindex,
                   [](OIIO::ImageSpec const &_this, char const *name) -> int {
                     return _this.channelindex(name);
                   }))
      .m(&OIIO::ImageSpec::roi)
      .m(&OIIO::ImageSpec::roi_full)
      .m(&OIIO::ImageSpec::set_roi)
      .m(&OIIO::ImageSpec::set_roi_full)
      .m(&OIIO::ImageSpec::set_colorspace)
      .m(&OIIO::ImageSpec::copy_dimensions)
      .m(&OIIO::ImageSpec::undefined);

  // As `ImageSpec` is opaque we need to declare a bunch of getters/setters
  // manually declare until babble does it for us automagically, one day.
  bbl::fn(&bblext::ImageSpec_get_x);
  bbl::fn(&bblext::ImageSpec_set_x);
  bbl::fn(&bblext::ImageSpec_get_y);
  bbl::fn(&bblext::ImageSpec_set_y);
  bbl::fn(&bblext::ImageSpec_get_z);
  bbl::fn(&bblext::ImageSpec_set_z);
  bbl::fn(&bblext::ImageSpec_get_width);
  bbl::fn(&bblext::ImageSpec_set_width);
  bbl::fn(&bblext::ImageSpec_get_height);
  bbl::fn(&bblext::ImageSpec_set_height);
  bbl::fn(&bblext::ImageSpec_get_depth);
  bbl::fn(&bblext::ImageSpec_set_depth);
  bbl::fn(&bblext::ImageSpec_get_full_x);
  bbl::fn(&bblext::ImageSpec_set_full_x);
  bbl::fn(&bblext::ImageSpec_get_full_y);
  bbl::fn(&bblext::ImageSpec_set_full_y);
  bbl::fn(&bblext::ImageSpec_get_full_z);
  bbl::fn(&bblext::ImageSpec_set_full_z);
  bbl::fn(&bblext::ImageSpec_get_full_width);
  bbl::fn(&bblext::ImageSpec_set_full_width);
  bbl::fn(&bblext::ImageSpec_get_full_height);
  bbl::fn(&bblext::ImageSpec_set_full_height);
  bbl::fn(&bblext::ImageSpec_get_full_depth);
  bbl::fn(&bblext::ImageSpec_set_full_depth);
  bbl::fn(&bblext::ImageSpec_get_tile_width);
  bbl::fn(&bblext::ImageSpec_set_tile_width);
  bbl::fn(&bblext::ImageSpec_get_tile_height);
  bbl::fn(&bblext::ImageSpec_set_tile_height);
  bbl::fn(&bblext::ImageSpec_get_tile_depth);
  bbl::fn(&bblext::ImageSpec_set_tile_depth);
  bbl::fn(&bblext::ImageSpec_get_nchannels);
  bbl::fn(&bblext::ImageSpec_set_nchannels);
  bbl::fn(&bblext::ImageSpec_get_format_basetype);
  bbl::fn(&bblext::ImageSpec_set_format_basetype);
  // bbl::fn(&bblext::ImageSpec_get_channelformats_ref);
  bbl::fn(&bblext::ImageSpec_set_channelformats);
  bbl::fn(&bblext::ImageSpec_get_alpha_channel);
  bbl::fn(&bblext::ImageSpec_set_alpha_channel);
  bbl::fn(&bblext::ImageSpec_get_z_channel);
  bbl::fn(&bblext::ImageSpec_set_z_channel);
  bbl::fn(&bblext::ImageSpec_get_deep);
  bbl::fn(&bblext::ImageSpec_set_deep);
  bbl::fn(&bblext::ImageSpec_clear_and_reserve_channelformats);
  bbl::fn(&bblext::ImageSpec_push_channelformat);
  bbl::fn(&bblext::ImageSpec_clear_and_reserve_channelnames);
  bbl::fn(&bblext::ImageSpec_push_channelname);
  bbl::fn(&bblext::ImageSpec_get_channelnames);

  bbl::Enum<OIIO::ImageSpec::SerialFormat>();
  bbl::Enum<OIIO::ImageSpec::SerialVerbose>();
}
