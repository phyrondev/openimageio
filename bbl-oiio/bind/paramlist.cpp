#include <babble>

#include <OpenImageIO/imagebuf.h>

BBL_MODULE(oiio) {

  bbl::Class<OIIO::ParamValue>()
      .ctor(bbl::Class<OIIO::ParamValue>::Ctor<>(), "default")
      .ctor(bbl::Class<OIIO::ParamValue>::Ctor<
                const OIIO::ustring &, OIIO::TypeDesc, int, const void *, bool>(
                "_name", "_type", "_nvalues", "_value", "_copy"),
            "ctor_01")
      .ctor(bbl::Class<OIIO::ParamValue>::Ctor<
                const OIIO::ustring &, OIIO::TypeDesc, int,
                OIIO::ParamValue::Interp, const void *, bool>(
                "_name", "_type", "_nvalues", "_interp", "_value", "_copy"),
            "ctor_02")
      // .ctor(bbl::Class<OIIO::ParamValue>::Ctor<string_view, TypeDesc, int,
      // const void *, bool>("_name", "_type", "_nvalues", "_value", "_copy"),
      // "ctor_03") .ctor(bbl::Class<OIIO::ParamValue>::Ctor<string_view,
      // TypeDesc, int, Interp, const void *, bool>("_name", "_type",
      // "_nvalues", "_interp", "_value", "_copy"), "ctor_04")
      // .ctor(bbl::Class<OIIO::ParamValue>::Ctor<string_view, int>("_name",
      // "value"), "ctor_05")
      // .ctor(bbl::Class<OIIO::ParamValue>::Ctor<string_view, float>("_name",
      // "value"), "ctor_06")
      // .ctor(bbl::Class<OIIO::ParamValue>::Ctor<string_view, ustring>("_name",
      // "value"), "ctor_07")
      // .ctor(bbl::Class<OIIO::ParamValue>::Ctor<string_view,
      // string_view>("_name", "value"), "ctor_08")
      // .ctor(bbl::Class<OIIO::ParamValue>::Ctor<string_view, TypeDesc,
      // string_view>("_name", "type", "value"), "ctor_09")
      // .ctor(bbl::Class<OIIO::ParamValue>::Ctor<const OIIO::ParamValue &,
      // bool>("p", "_copy"), "ctor_10")
      .m((void(OIIO::ParamValue::*)(OIIO::ustring, OIIO::TypeDesc, int,
                                    OIIO::ParamValue::Interp, const void *,
                                    bool)) &
             OIIO::ParamValue::init,
         "init_00")
      .m((void(OIIO::ParamValue::*)(OIIO::ustring, OIIO::TypeDesc, int,
                                    const void *, bool)) &
             OIIO::ParamValue::init,
         "init_01")
      // .m((void (OIIO::ParamValue::*)(string_view, OIIO::TypeDesc, int, const
      // void *, bool))
      //     &OIIO::ParamValue::init, "init_02")
      // .m((void (OIIO::ParamValue::*)(string_view, OIIO::TypeDesc, int,
      // OIIO::ParamValue::Interp, const void *, bool))
      //     &OIIO::ParamValue::init, "init_03")
      .m((const OIIO::ParamValue &(
             OIIO::ParamValue::*)(const OIIO::ParamValue &)) &
             OIIO::ParamValue::operator=,
         "op_assign_00")
      .m((const OIIO::ParamValue &(OIIO::ParamValue::*)(OIIO::ParamValue &&)) &
             OIIO::ParamValue::operator=,
         "op_assign_01")
      .m(&OIIO::ParamValue::name)
      .m(&OIIO::ParamValue::uname)
      .m(&OIIO::ParamValue::type)
      .m(&OIIO::ParamValue::nvalues)
      .m(&OIIO::ParamValue::data)
      .m(&OIIO::ParamValue::datasize)
      .m((OIIO::ParamValue::Interp(OIIO::ParamValue::*)() const) &
             OIIO::ParamValue::interp,
         "interp_00")
      .m((void(OIIO::ParamValue::*)(OIIO::ParamValue::Interp)) &
             OIIO::ParamValue::interp,
         "interp_01")
      .m(&OIIO::ParamValue::is_nonlocal)
      .m(&OIIO::ParamValue::get_int)
      .m(&OIIO::ParamValue::get_int_indexed)
      .m(&OIIO::ParamValue::get_float)
      .m(&OIIO::ParamValue::get_float_indexed)
      .m(&OIIO::ParamValue::get_string)
      .m(&OIIO::ParamValue::get_string_indexed)
      .m(&OIIO::ParamValue::get_ustring)
      .m(&OIIO::ParamValue::get_ustring_indexed);

  bbl::Enum<OIIO::ParamValue::Interp>();

  bbl::Class<OIIO::ParamValueList>()
      .ctor(bbl::Class<OIIO::ParamValueList>::Ctor<>(), "default")
      .m(&OIIO::ParamValueList::grow)
      .m(bbl::Wrap((OIIO::ParamValue *
                    (OIIO::ParamValueList::*)(OIIO::string_view, OIIO::TypeDesc,
                                              bool)) &
                       OIIO::ParamValueList::find_pv,
                   [](OIIO::ParamValueList &_this, char const *name,
                      OIIO::TypeDesc type,
                      bool casesensitive) -> OIIO::ParamValue * {
                     return _this.find_pv(name, type, casesensitive);
                   }),
         "find")
      .m(bbl::Wrap((OIIO::ParamValue const *(
                       OIIO::ParamValueList::*)(OIIO::string_view,
                                                OIIO::TypeDesc, bool) const) &
                       OIIO::ParamValueList::find_pv,
                   [](OIIO::ParamValueList const &_this, char const *name,
                      OIIO::TypeDesc type,
                      bool casesensitive) -> OIIO::ParamValue const * {
                     return _this.find_pv(name, type, casesensitive);
                   }),
         "find_const")
      .m(bbl::Wrap(&OIIO::ParamValueList::get_int,
                   [](OIIO::ParamValueList const &_this, char const *name,
                      int defaultval, bool casesensitive, bool convert) -> int {
                     return _this.get_int(name, defaultval, casesensitive,
                                          convert);
                   }))
      .m(bbl::Wrap(
          &OIIO::ParamValueList::get_float,
          [](OIIO::ParamValueList const &_this, char const *name,
             float defaultval, bool casesensitive, bool convert) -> float {
            return _this.get_float(name, defaultval, casesensitive, convert);
          }))
      .m(bbl::Wrap(&OIIO::ParamValueList::get_string,
                   [](OIIO::ParamValueList const &_this, char const *name,
                      char const *defaultval, bool casesensitive, bool convert,
                      char const **result, long long *len) -> void {
                     auto sv = _this.get_string(name, defaultval, casesensitive,
                                                convert);
                     *result = sv.c_str();
                     *len = sv.size();
                   }))
      .m(bbl::Wrap(&OIIO::ParamValueList::remove,
                   [](OIIO::ParamValueList &_this, char const *name,
                      OIIO::TypeDesc type, bool casesensitive) -> void {
                     _this.remove(name, type, casesensitive);
                   }))
      .m(bbl::Wrap(&OIIO::ParamValueList::contains,
                   [](OIIO::ParamValueList &_this, char const *name,
                      OIIO::TypeDesc type, bool casesensitive) -> bool {
                     return _this.contains(name, type, casesensitive);
                   }))
      .m((void(OIIO::ParamValueList::*)(const OIIO::ParamValue &, bool)) &
             OIIO::ParamValueList::add_or_replace,
         "add_or_replace")
      .m(bbl::Wrap((void(OIIO::ParamValueList::*)(
                       OIIO::string_view, OIIO::TypeDesc, int, const void *)) &
                       OIIO::ParamValueList::attribute,
                   [](OIIO::ParamValueList &_this, char const *name,
                      OIIO::TypeDesc type, int nvalues, void const *data)
                       -> void { _this.attribute(name, type, nvalues, data); }),
         "attribute")
      .m(bbl::Wrap(&OIIO::ParamValueList::getattributetype,
                   [](OIIO::ParamValueList const &_this, char const *name,
                      bool casesensitive) -> OIIO::TypeDesc {
                     return _this.getattributetype(name, casesensitive);
                   }))
      .m(bbl::Wrap(
             (bool(OIIO::ParamValueList::*)(OIIO::string_view, OIIO::TypeDesc,
                                            void *, bool) const) &
                 OIIO::ParamValueList::getattribute,
             [](OIIO::ParamValueList const &_this, char const *name,
                OIIO::TypeDesc type, void *value, bool casesensitive) -> bool {
               return _this.getattribute(name, type, value, casesensitive);
             }),
         "getattribute")
      .m(&OIIO::ParamValueList::sort)
      .m(&OIIO::ParamValueList::merge)
      .m(&OIIO::ParamValueList::free)
      // .m((ParamValue & (OIIO::ParamValueList::*)(int))
      //     &OIIO::ParamValueList::operator[], "op_index_00")
      // .m((const OIIO::ParamValue & (OIIO::ParamValueList::*)(int) const)
      //     &OIIO::ParamValueList::operator[], "op_index_01")
      // .m((AttrDelegate<const OIIO::ParamValueList>
      // (OIIO::ParamValueList::*)(string_view) const)
      //     &OIIO::ParamValueList::operator[], "op_index_02")
      // .m((AttrDelegate<ParamValueList>
      // (OIIO::ParamValueList::*)(string_view))
      //     &OIIO::ParamValueList::operator[], "op_index_03")
      // .m((ParamValueList & (OIIO::ParamValueList::*)(const
      // OIIO::ParamValueList &))
      //     &OIIO::ParamValueList::operator=, "op_assign_00")
      // .m((ParamValueList & (OIIO::ParamValueList::*)(ParamValueList &&))
      //     &OIIO::ParamValueList::operator=, "op_assign_01")
      ;
}
