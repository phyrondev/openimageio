#include <babble>

#include <OpenImageIO/ustring.h>

BBL_MODULE(oiio) {

  bbl::Class<OIIO::ustring>()
      // .replace_with<ustr>()
      .ctor(bbl::Class<OIIO::ustring>::Ctor<>(), "default")
      .ctor(bbl::Class<OIIO::ustring>::Ctor<const char *>("str"), "new")
      .ctor(bbl::Class<OIIO::ustring>::Ctor<const char *, size_t>("str", "len"),
            "new_from_parts")
      .m(&OIIO::ustring::c_str)
      .m(&OIIO::ustring::data)
      .m(&OIIO::ustring::hash)
      .m(&OIIO::ustring::size)
      .m(&OIIO::ustring::empty)
      // .m((bool (OIIO::ustring::*)(const OIIO::ustring &) const)
      //     &OIIO::ustring::operator==, "op_eq")
      // .m((bool (OIIO::ustring::*)(const OIIO::ustring &) const)
      //     &OIIO::ustring::operator!=, "op_neq")
      // .m((bool (OIIO::ustring::*)(const char *) const)
      //     &OIIO::ustring::operator!=, "op_neq_c_str")
      .m(&OIIO::ustring::operator<, "op_lt")
      // .m(&OIIO::ustring::getstats)
      // .m(&OIIO::ustring::memory)
      // .m(&OIIO::ustring::total_ustrings)
      // .m(&OIIO::ustring::is_unique)
      // .m(&OIIO::ustring::from_unique)
      ;
}
