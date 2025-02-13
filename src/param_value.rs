use crate::*;
use std::{marker::PhantomData, num::NonZeroU32};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Interpolation {
    Constant = oiio_Interp::oiio_Interp_INTERP_CONSTANT.0 as _,
    PerPiece = oiio_Interp::oiio_Interp_INTERP_PERPIECE.0 as _,
    Linear = oiio_Interp::oiio_Interp_INTERP_LINEAR.0 as _,
    Vertex = oiio_Interp::oiio_Interp_INTERP_VERTEX.0 as _,
}

pub trait ValueTypeDesc<T> {
    fn type_desc(value: &T) -> TypeDesc;
}

/*
impl ValueTypeDesc<u8> for u8 {
    fn type_desc(_: &u8) -> TypeDesc {
        TypeDesc {
            base_type: Some(BaseType::U8),
            aggregate: Aggregate::Scalar,
            vec_semantics: None,
            array_len: None,
        }
    }
}

impl ValueTypeDesc<&[u8]> for &[u8] {
    fn type_desc(value: &&[u8]) -> TypeDesc {
        TypeDesc {
            base_type: Some(BaseType::U8),
            aggregate: Aggregate::Scalar,
            vec_semantics: None,
            array_len: Some(ArrayLen::Specific(NonZeroU32::new(value.len()).unwrap())),,
        }
    }
}*/

macro_rules! value_type_desc {
    ($type:ident, $base_type:expr, $aggregate:expr) => {
        impl ValueTypeDesc<$type> for $type {
            fn type_desc(_: &$type) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Scalar,
                    vec_semantics: None,
                    array_len: None,
                }
            }
        }

        // Array types

        impl ValueTypeDesc<&[$type]> for &[$type] {
            fn type_desc(value: &&[$type]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Scalar,
                    vec_semantics: None,
                    array_len: Some(ArrayLen::Specific(
                        NonZeroU32::new(value.len().try_into().unwrap()).unwrap(),
                    )),
                }
            }
        }

        impl ValueTypeDesc<[$type; 2]> for [$type; 2] {
            fn type_desc(_: &[$type; 2]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Vec2,
                    vec_semantics: None,
                    array_len: None,
                }
            }
        }

        impl ValueTypeDesc<[$type; 3]> for [$type; 3] {
            fn type_desc(_: &[$type; 3]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Vec3,
                    vec_semantics: None,
                    array_len: None,
                }
            }
        }

        impl ValueTypeDesc<[$type; 4]> for [$type; 4] {
            fn type_desc(_: &[$type; 4]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Vec4,
                    vec_semantics: None,
                    array_len: None,
                }
            }
        }

        impl ValueTypeDesc<[$type; 9]> for [$type; 9] {
            fn type_desc(_: &[$type; 9]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Matrix3,
                    vec_semantics: None,
                    array_len: None,
                }
            }
        }

        impl ValueTypeDesc<[$type; 16]> for [$type; 16] {
            fn type_desc(_: &[$type; 16]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Matrix4,
                    vec_semantics: None,
                    array_len: None,
                }
            }
        }

        impl ValueTypeDesc<&[[$type; 2]]> for &[[$type; 2]] {
            fn type_desc(value: &&[[$type; 2]]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Vec2,
                    vec_semantics: None,
                    array_len: Some(ArrayLen::Specific(
                        NonZeroU32::new(value.len().try_into().unwrap()).unwrap(),
                    )),
                }
            }
        }

        impl ValueTypeDesc<&[[$type; 3]]> for &[[$type; 3]] {
            fn type_desc(value: &&[[$type; 3]]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Vec3,
                    vec_semantics: None,
                    array_len: Some(ArrayLen::Specific(
                        NonZeroU32::new(value.len().try_into().unwrap()).unwrap(),
                    )),
                }
            }
        }

        impl ValueTypeDesc<&[[$type; 4]]> for &[[$type; 4]] {
            fn type_desc(value: &&[[$type; 4]]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Vec4,
                    vec_semantics: None,
                    array_len: Some(ArrayLen::Specific(
                        NonZeroU32::new(value.len().try_into().unwrap()).unwrap(),
                    )),
                }
            }
        }

        impl ValueTypeDesc<&[[$type; 9]]> for &[[$type; 9]] {
            fn type_desc(value: &&[[$type; 9]]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Matrix3,
                    vec_semantics: None,
                    array_len: Some(ArrayLen::Specific(
                        NonZeroU32::new(value.len().try_into().unwrap()).unwrap(),
                    )),
                }
            }
        }

        impl ValueTypeDesc<&[[$type; 16]]> for &[[$type; 16]] {
            fn type_desc(value: &&[[$type; 16]]) -> TypeDesc {
                TypeDesc {
                    base_type: Some($base_type),
                    aggregate: Aggregate::Matrix4,
                    vec_semantics: None,
                    array_len: Some(ArrayLen::Specific(
                        NonZeroU32::new(value.len().try_into().unwrap()).unwrap(),
                    )),
                }
            }
        }
    };
}

value_type_desc!(bool, BaseType::U8, Aggregate::Scalar);
value_type_desc!(u8, BaseType::U8, Aggregate::Scalar);
value_type_desc!(u16, BaseType::U16, Aggregate::Scalar);
value_type_desc!(u32, BaseType::U32, Aggregate::Scalar);
value_type_desc!(u64, BaseType::U64, Aggregate::Scalar);
value_type_desc!(f32, BaseType::F32, Aggregate::Scalar);
value_type_desc!(f64, BaseType::F64, Aggregate::Scalar);
value_type_desc!(i8, BaseType::I8, Aggregate::Scalar);
value_type_desc!(i16, BaseType::I16, Aggregate::Scalar);
value_type_desc!(i32, BaseType::I32, Aggregate::Scalar);
value_type_desc!(i64, BaseType::I64, Aggregate::Scalar);

impl ValueTypeDesc<&str> for &str {
    fn type_desc(_: &&str) -> TypeDesc {
        TypeDesc {
            base_type: Some(BaseType::String),
            aggregate: Aggregate::Scalar,
            vec_semantics: None,
            array_len: None,
        }
    }
}

impl ValueTypeDesc<&[&str]> for &[&str] {
    fn type_desc(value: &&[&str]) -> TypeDesc {
        TypeDesc {
            base_type: Some(BaseType::String),
            aggregate: Aggregate::Scalar,
            vec_semantics: None,
            array_len: Some(ArrayLen::Specific(
                NonZeroU32::new(value.len().try_into().unwrap()).unwrap(),
            )),
        }
    }
}

/// Holds a named parameter and typed data.
///
/// Usually, it owns the data (holding it in the struct itself if small enough,
/// dynamically allocated for larger things), but it can also refer to non-owned
/// data.
///
/// The data is usually a single value of any type described by [`TypeDesc`]
/// (including slices). It may also hold more than one value of the type --
/// this is usually only used in a geometric context, such as storing a value
/// for each vertex in a mesh. Please note the subtle distinction between the
/// value type itself being an array, versus having multiple values as a
/// parameter, versus the type of the value having multiple components (such
/// as a point or color). Any combination of these may be present.
///
/// To clarify, if you have an array of 4 colors for each of 15 mesh vertices,
/// that means:
///
///  - There are 15 *values* (one for each vertex).
///  - Each value has an array of 4 *elements*.
///  - Each element is a *color*.
///  - A color has 3 *components* (R, G, B)
///
/// Thus, it would be constructed as:
///
/// ```
/// #  use openimageio::{ParamValue, ParamValueOptions, VecSemantics};
/// let red_times_15 = ParamValue::new_with(
///     "mycolor",
///     &[[[1.0, 0.0, 0.0]; 4]; 15],
///     &ParamValueOptions {
///         vec_semantics: Some(VecSemantics::Color),
///         ..Default::default()
///     },
/// );
/// ```
///
/// # Examples
///
/// ```
/// # use openimageio::ParamValue;
/// // Single int:
/// let my_i32 = 42i32;
/// ParamValue::new("foo", my_i32);
///
/// // *Three* `u32` values (say, one per vertex index of a triangle):
/// let my_u32_array = [1u32, 2, 3];
/// let multi = ParamValue::new_multi("foo", &my_u32_array);
///
/// // A *single* value which is an array of *three* `u32`s:
/// let simgle = ParamValue::new("baz", &my_u32_array);
/// ```
///
/// Single values:
///
/// ```
/// # use openimageio::ParamValue;
/// // single `i32`.
/// let foo = ParamValue::new("foo", 42i32);
///
/// // single `f32`
/// let bar = ParamValue::new("bar", 42.0f32);
///
/// // single String
/// let baz = ParamValue::new("baz", "forty two");
/// ```
pub struct ParamValue {
    ptr: *mut oiio_ParamValue_t,
}

pub struct ParamValueOptions {
    pub vec_semantics: Option<VecSemantics>,
    pub interpolation: Interpolation,
}

impl Default for ParamValueOptions {
    fn default() -> Self {
        Self {
            vec_semantics: None,
            interpolation: Interpolation::Constant,
        }
    }
}

impl ParamValue {
    pub fn new<T: ValueTypeDesc<T>>(name: &str, value: T) -> Self {
        let type_desc = T::type_desc(&value);

        Self::new_ffi(name, value, type_desc, 1, &ParamValueOptions::default())
    }

    pub fn new_multi<T: ValueTypeDesc<T> + AsRef<[T]>>(name: &str, value: T) -> Self {
        let mut type_desc = T::type_desc(&value);
        type_desc.array_len = None;

        let len = value.as_ref().len();
        Self::new_ffi(name, value, type_desc, len, &ParamValueOptions::default())
    }

    pub fn new_with<T: ValueTypeDesc<T>, const N: usize>(
        name: &str,
        value: T,
        options: &ParamValueOptions,
    ) -> Self {
        let mut type_desc = T::type_desc(&value);
        type_desc.vec_semantics = options.vec_semantics;

        Self::new_ffi(name, value, type_desc, 1, options)
    }

    pub fn new_multi_with<'a, T: ValueTypeDesc<T> + AsRef<&'a [T]> + 'a, const N: usize>(
        name: &str,
        value: T,
        options: &ParamValueOptions,
    ) -> Self {
        let mut type_desc = T::type_desc(&value);
        type_desc.vec_semantics = options.vec_semantics;
        type_desc.array_len = None;

        let len = value.as_ref().len();
        Self::new_ffi(name, value, type_desc, len, options)
    }
}

impl ParamValue {
    pub(crate) fn new_ffi<T>(
        name: &str,
        value: T,
        type_desc: TypeDesc,
        len: usize,
        options: &ParamValueOptions,
    ) -> Self {
        Self {
            ptr: {
                let mut ptr = std::mem::MaybeUninit::<*mut oiio_ParamValue_t>::uninit();

                unsafe {
                    oiio_ParamValue_ctor(
                        Ustring::from(name).as_raw_ptr(),
                        type_desc.into(),
                        len as _,
                        transmute(options.interpolation as u32),
                        &raw const value as _,
                        true, // Copy
                        &raw mut ptr as _,
                    );

                    ptr.assume_init()
                }
            },
        }
    }
}

impl Drop for ParamValue {
    fn drop(&mut self) {
        unsafe { oiio_ParamValue_dtor(self.ptr) };
    }
}

impl ParamValue {
    pub(crate) fn as_raw_ptr(&self) -> *const oiio_ParamValue_t {
        self.ptr
    }

    #[allow(dead_code)]
    pub(crate) fn as_raw_ptr_mut(&mut self) -> *mut oiio_ParamValue_t {
        self.ptr
    }
}

pub struct ParamValueList {
    ptr: *mut oiio_ParamValueList_t,
}

impl Default for ParamValueList {
    fn default() -> Self {
        Self::new()
    }
}

impl ParamValueList {
    pub fn new() -> Self {
        Self {
            ptr: {
                let mut ptr = std::mem::MaybeUninit::<*mut oiio_ParamValueList_t>::uninit();

                unsafe {
                    oiio_ParamValueList_default(&raw mut ptr as _);

                    ptr.assume_init()
                }
            },
        }
    }

    pub fn add_or_replace(&mut self, param_value: ParamValue) {
        unsafe {
            oiio_ParamValueList_add_or_replace(self.ptr, param_value.as_raw_ptr(), false);
        }
    }

    pub fn add_or_replace_ignore_case(&mut self, param_value: ParamValue) {
        unsafe {
            oiio_ParamValueList_add_or_replace(self.ptr, param_value.as_raw_ptr(), true);
        }
    }
}

impl ParamValueList {
    pub(crate) fn as_raw_ptr(&self) -> *const oiio_ParamValueList_t {
        self.ptr
    }

    #[allow(dead_code)]
    pub(crate) fn as_raw_ptr_mut(&mut self) -> *mut oiio_ParamValueList_t {
        self.ptr
    }
}

impl Drop for ParamValueList {
    fn drop(&mut self) {
        unsafe { oiio_ParamValueList_dtor(self.ptr) };
    }
}

pub struct ParamValueSlice<'a> {
    ptr: *mut oiio_ParamValueSpan_t,
    _marker: PhantomData<&'a ParamValueList>,
}

impl<'a> From<&'a ParamValueList> for ParamValueSlice<'a> {
    fn from(param_value_list: &ParamValueList) -> Self {
        Self {
            ptr: {
                let mut ptr = std::mem::MaybeUninit::<*mut oiio_ParamValueSpan_t>::uninit();

                unsafe {
                    oiio_ParamValueSpan_ctor(param_value_list.as_raw_ptr(), &raw mut ptr as _);

                    ptr.assume_init()
                }
            },
            _marker: PhantomData,
        }
    }
}

impl ParamValueSlice<'_> {
    pub(crate) fn as_raw_ptr(&self) -> *const oiio_ParamValueSpan_t {
        self.ptr
    }

    #[allow(dead_code)]
    pub(crate) fn as_raw_ptr_mut(&mut self) -> *mut oiio_ParamValueSpan_t {
        self.ptr
    }
}

impl Drop for ParamValueSlice<'_> {
    fn drop(&mut self) {
        unsafe { oiio_ParamValueSpan_dtor(self.ptr) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_value() {
        let my_i32 = 42i32;
        let _my_i32 = ParamValue::new("my_i32", my_i32);

        // *Three* `u32` values (say, one per vertex index of a triangle):
        //let my_u32_array = [1u32, 2, 3];
        //ParamValue::new_multi("foo", my_u32_array);

        // A *single* value which is an array of *three* `u32`s:
        //ParamValue::new("baz", &my_u32_array);

        // single `i32`.
        let _foo = ParamValue::new("foo", 42i32);

        // single `f32`
        let _bar = ParamValue::new("bar", 42.0f32);

        // single String
        let _baz = ParamValue::new("baz", "forty two");
    }

    #[test]
    fn param_value_list() {
        let mut param_value_list = ParamValueList::new();

        param_value_list.add_or_replace(ParamValue::new("foo", 42i32));
        param_value_list.add_or_replace(ParamValue::new("bar", 42.0f32));
        param_value_list.add_or_replace(ParamValue::new("baz", "forty two"));

        let _param_value_slice = ParamValueSlice::from(&param_value_list);
    }
}
