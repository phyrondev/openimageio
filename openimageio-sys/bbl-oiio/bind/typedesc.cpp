#include <babble-std>
#include <babble>

#include <OpenImageIO/imagebuf.h>

BBL_MODULE(oiio) {

  bbl::Class<OIIO::TypeDesc>()
      .value_type()
      .f(&OIIO::TypeDesc::basetype)
      .f(&OIIO::TypeDesc::aggregate)
      .f(&OIIO::TypeDesc::vecsemantics)
      .f(&OIIO::TypeDesc::reserved)
      .f(&OIIO::TypeDesc::arraylen)
      .ctor(bbl::Class<OIIO::TypeDesc>::Ctor<OIIO::TypeDesc::BASETYPE,
                                             OIIO::TypeDesc::AGGREGATE,
                                             OIIO::TypeDesc::VECSEMANTICS, int>(
                "btype", "agg", "semantics", "arraylen"),
            "ctor_00")
      .ctor(bbl::Class<OIIO::TypeDesc>::Ctor<OIIO::TypeDesc::BASETYPE, int>(
                "btype", "arraylen"),
            "ctor_01")
      .ctor(bbl::Class<OIIO::TypeDesc>::Ctor<OIIO::TypeDesc::BASETYPE,
                                             OIIO::TypeDesc::AGGREGATE, int>(
                "btype", "agg", "arraylen"),
            "ctor_02")
      // .ctor(bbl::Class<OIIO::TypeDesc>::Ctor<string_view>("typestring"),
      // "ctor_03")
      .m(&OIIO::TypeDesc::c_str)
      .m(&OIIO::TypeDesc::numelements)
      .m(&OIIO::TypeDesc::basevalues)
      .m(&OIIO::TypeDesc::is_array)
      .m(&OIIO::TypeDesc::is_unsized_array)
      .m(&OIIO::TypeDesc::is_sized_array)
      .m(&OIIO::TypeDesc::size)
      .m(&OIIO::TypeDesc::elementtype)
      .m(&OIIO::TypeDesc::elementsize)
      .m(&OIIO::TypeDesc::scalartype)
      .m(&OIIO::TypeDesc::basesize)
      .m(&OIIO::TypeDesc::is_floating_point)
      .m(&OIIO::TypeDesc::is_signed)
      .m(&OIIO::TypeDesc::is_unknown)
      .m(bbl::Wrap(&OIIO::TypeDesc::fromstring,
                   [](OIIO::TypeDesc &_this, char const *typestring) -> size_t {
                     return _this.fromstring(typestring);
                   }))
      .m(&OIIO::TypeDesc::operator==, "op_eq")
      // .m(&OIIO::TypeDesc::operator!=, "op_neq")
      .m(&OIIO::TypeDesc::equivalent)
      .m(&OIIO::TypeDesc::is_vec2)
      .m(&OIIO::TypeDesc::is_vec3)
      .m(&OIIO::TypeDesc::is_vec4)
      .m(&OIIO::TypeDesc::is_box2)
      .m(&OIIO::TypeDesc::is_box3)
      .m(&OIIO::TypeDesc::unarray);
  // .m(&OIIO::TypeDesc::operator<, "op_lt")
  // .m(&OIIO::TypeDesc::basetype_merge);

  bbl::Enum<OIIO::TypeDesc::AGGREGATE>();
  bbl::Enum<OIIO::TypeDesc::BASETYPE>();
  bbl::Enum<OIIO::TypeDesc::VECSEMANTICS>();

  bbl::Class<std::vector<OIIO::TypeDesc>>("VecTypeDesc")
      BBL_STD_VECTOR_METHODS(OIIO::TypeDesc);
}
