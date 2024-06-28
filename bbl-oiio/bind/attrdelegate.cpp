#include <babble>

#include <OpenImageIO/imagebuf.h>

BBL_MODULE(oiio) {
#if 0
    /// TODO: instantiate this template
    bbl::Class<OpenImageIO_v2_3::AttrDelegate>()
        .ctor(bbl::Class<OpenImageIO_v2_3::AttrDelegate>::Ctor<C *, string_view>("obj", "name"), "ctor_00")
        /** TODO: instantiate this template
        .m((const T & (OpenImageIO_v2_3::AttrDelegate::*)(const T &))
            &OpenImageIO_v2_3::AttrDelegate::operator=, "op_assign_00")
        */
        /** TODO: instantiate this template
        .m((const T & (OpenImageIO_v2_3::AttrDelegate::*)(const T &))
            &OpenImageIO_v2_3::AttrDelegate::operator=, "op_assign_01")
        */
        .m((const char * (OpenImageIO_v2_3::AttrDelegate::*)(const char *))
            &OpenImageIO_v2_3::AttrDelegate::operator=, "op_assign_02")
        .m(&OpenImageIO_v2_3::AttrDelegate::type)
        /** TODO: instantiate this template
        .m((T (OpenImageIO_v2_3::AttrDelegate::*)(const T &) const)
            &OpenImageIO_v2_3::AttrDelegate::get, "get_00")
        */
        /** TODO: instantiate this template
        .m((T (OpenImageIO_v2_3::AttrDelegate::*)(const T &) const)
            &OpenImageIO_v2_3::AttrDelegate::get, "get_01")
        */
        /** TODO: instantiate this template
        .m((T (OpenImageIO_v2_3::AttrDelegate::*)(int, const T &) const)
            &OpenImageIO_v2_3::AttrDelegate::get_indexed, "get_indexed_00")
        */
        /** TODO: instantiate this template
        .m((T (OpenImageIO_v2_3::AttrDelegate::*)(int, const T &) const)
            &OpenImageIO_v2_3::AttrDelegate::get_indexed, "get_indexed_01")
        */
        .m(&OpenImageIO_v2_3::AttrDelegate::as_string)
        /** TODO: instantiate this template
        .m(&OpenImageIO_v2_3::AttrDelegate::as_vec)
        */
        .m(&OpenImageIO_v2_3::AttrDelegate::operator basic_string)
    ;
#endif


}
