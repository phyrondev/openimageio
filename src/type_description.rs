use crate::*;
use core::num::NonZeroU32;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Describes the base data types that correspond (mostly) to the Rust
/// primitive/`std` types.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BaseType {
    //Unknown = oiio_BASETYPE::oiio_BASETYPE_UNKNOWN.0 as _,
    #[default]
    U8 = oiio_BASETYPE::oiio_BASETYPE_UINT8.0 as _,

    I8 = oiio_BASETYPE::oiio_BASETYPE_INT8.0 as _,
    U16 = oiio_BASETYPE::oiio_BASETYPE_UINT16.0 as _,
    I16 = oiio_BASETYPE::oiio_BASETYPE_INT16.0 as _,
    U32 = oiio_BASETYPE::oiio_BASETYPE_UINT32.0 as _,
    I32 = oiio_BASETYPE::oiio_BASETYPE_INT32.0 as _,
    I64 = oiio_BASETYPE::oiio_BASETYPE_INT64.0 as _,
    U64 = oiio_BASETYPE::oiio_BASETYPE_UINT64.0 as _,

    F16 = oiio_BASETYPE::oiio_BASETYPE_HALF.0 as _,
    F32 = oiio_BASETYPE::oiio_BASETYPE_FLOAT.0 as _,
    F64 = oiio_BASETYPE::oiio_BASETYPE_DOUBLE.0 as _,

    String = oiio_BASETYPE::oiio_BASETYPE_STRING.0 as _,

    Ptr = oiio_BASETYPE::oiio_BASETYPE_PTR.0 as _,
}

impl From<BaseType> for oiio_BASETYPE {
    fn from(base_type: BaseType) -> oiio_BASETYPE {
        match base_type {
            BaseType::U8 => oiio_BASETYPE::oiio_BASETYPE_UINT8,
            BaseType::I8 => oiio_BASETYPE::oiio_BASETYPE_INT8,
            BaseType::U16 => oiio_BASETYPE::oiio_BASETYPE_UINT16,
            BaseType::I16 => oiio_BASETYPE::oiio_BASETYPE_INT16,
            BaseType::U32 => oiio_BASETYPE::oiio_BASETYPE_UINT32,
            BaseType::I32 => oiio_BASETYPE::oiio_BASETYPE_INT32,
            BaseType::I64 => oiio_BASETYPE::oiio_BASETYPE_INT64,
            BaseType::U64 => oiio_BASETYPE::oiio_BASETYPE_UINT64,
            BaseType::F16 => oiio_BASETYPE::oiio_BASETYPE_HALF,
            BaseType::F32 => oiio_BASETYPE::oiio_BASETYPE_FLOAT,
            BaseType::F64 => oiio_BASETYPE::oiio_BASETYPE_DOUBLE,
            BaseType::String => oiio_BASETYPE::oiio_BASETYPE_STRING,
            BaseType::Ptr => oiio_BASETYPE::oiio_BASETYPE_PTR,
        }
    }
}

impl TryFrom<oiio_BASETYPE> for BaseType {
    type Error = ();

    fn try_from(base_type: oiio_BASETYPE) -> Result<BaseType, Self::Error> {
        match base_type {
            oiio_BASETYPE::oiio_BASETYPE_NONE => Err(()),
            _ => Ok(match base_type {
                oiio_BASETYPE::oiio_BASETYPE_UINT8 => BaseType::U8,
                oiio_BASETYPE::oiio_BASETYPE_INT8 => BaseType::I8,
                oiio_BASETYPE::oiio_BASETYPE_UINT16 => BaseType::U16,
                oiio_BASETYPE::oiio_BASETYPE_INT16 => BaseType::I16,
                oiio_BASETYPE::oiio_BASETYPE_UINT32 => BaseType::U32,
                oiio_BASETYPE::oiio_BASETYPE_INT32 => BaseType::I32,
                oiio_BASETYPE::oiio_BASETYPE_INT64 => BaseType::I64,
                oiio_BASETYPE::oiio_BASETYPE_UINT64 => BaseType::U64,
                oiio_BASETYPE::oiio_BASETYPE_HALF => BaseType::F16,
                oiio_BASETYPE::oiio_BASETYPE_FLOAT => BaseType::F32,
                oiio_BASETYPE::oiio_BASETYPE_DOUBLE => BaseType::F64,
                oiio_BASETYPE::oiio_BASETYPE_STRING => BaseType::String,
                oiio_BASETYPE::oiio_BASETYPE_PTR => BaseType::Ptr,
                _ => unreachable!(),
            }),
        }
    }
}

/// Describes whether a [`TypeDesc`] is a simple scalar of one of the
/// [`BaseType`]s, or one of several simple aggregates.
///
/// Note that *aggregates* and *arrays* are different.
///
/// * An array of three `f32`s: ```ignore TypeDescription { base_type:
///   Some(BaseType::F32), array_len: Some(3), ..Default::default() } ```
///
/// * A single three-component vector comprised of `f32`s.
///
///   ```ignore
///   TypeDescription {
///       base_type: Some(BaseType::F32),
///       aggregate: Aggregate::Vec3,
///       ..Default::default()
///   }
///   ```
///
/// * An array of three vectors, each of which is comprised of three `f32`s:
///
///   ```ignore
///   TypeDescription {
///       base_type: Some(BaseType::F32),
///       array_len: Some(3),
///       aggregate: Aggregate::Vec3,
///       ..Default::default()
///   }
///   ```
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Aggregate {
    #[default]
    Scalar = oiio_AGGREGATE::oiio_AGGREGATE_SCALAR.0 as _,
    /// Two values representing a 2D vector.
    Vec2 = oiio_AGGREGATE::oiio_AGGREGATE_VEC2.0 as _,
    /// Three values representing a 3D vector.
    Vec3 = oiio_AGGREGATE::oiio_AGGREGATE_VEC3.0 as _,
    /// Four values representing a 4D vector.
    Vec4 = oiio_AGGREGATE::oiio_AGGREGATE_VEC4.0 as _,
    /// Nine values representing a 3×3 matrix.
    Matrix3 = oiio_AGGREGATE::oiio_AGGREGATE_MATRIX33.0 as _,
    /// 16 values representing a 4×4 matrix.
    Matrix4 = oiio_AGGREGATE::oiio_AGGREGATE_MATRIX44.0 as _,
}

/// A hint about what the data represent(s).
///
/// For example, if a spatial vector quantity should transform as a point,
/// direction vector, or surface normal.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VecSemantics {
    /// A color.
    #[default]
    Color = oiio_VECSEMANTICS::oiio_VECSEMANTICS_COLOR.0 as _,
    /// A spatial location.
    Point = oiio_VECSEMANTICS::oiio_VECSEMANTICS_POINT.0 as _,
    /// A spatial direction.
    Vector = oiio_VECSEMANTICS::oiio_VECSEMANTICS_VECTOR.0 as _,
    /// A surface normal.
    Normal = oiio_VECSEMANTICS::oiio_VECSEMANTICS_NORMAL.0 as _,
    /// An `u8[4]` representing the standard four byte encoding of an
    /// [SMPTE timecode](https://en.wikipedia.org/wiki/SMPTE_timecode).
    TimeCode = oiio_VECSEMANTICS::oiio_VECSEMANTICS_TIMECODE.0 as _,
    /// An `u8[28]` representing the standard 28 byte encoding of an SMPTE
    /// keycode.
    KeyCode = oiio_VECSEMANTICS::oiio_VECSEMANTICS_KEYCODE.0 as _,
    /// A [`Aggregate::Vec2`] representing a rational number -- `val[0]` ÷
    /// `val[1]`.
    Rational = oiio_VECSEMANTICS::oiio_VECSEMANTICS_RATIONAL.0 as _,
    /// An [`[Aggregate::Vec2; 2]`](Aggregate::Vec2) or
    /// [`[Aggregate::Vec3; 2]`](Aggregate::Vec3) that represents a 2D or 3D
    /// bounding box (min/max).
    Box = oiio_VECSEMANTICS::oiio_VECSEMANTICS_BOX.0 as _,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum ArrayLen {
    Specific(NonZeroU32),
    #[default]
    Unspecific,
}

/// Describes the types of data that are handled by OpenImageIO.
///
/// There are two kinds of data that are important to OpenImageIO:
///
/// * Internal data is in the memory of the computer, used by an application
///   program.
///
/// * Native file data is what is stored in an image file itself (i.e., on the
///   "other side" of the abstraction layer that OpenImageIO provides).
///
/// Both internal and file data is stored in a particular data format that
/// describes the numerical encoding of the values. OpenImageIO understands
/// several types of data encodings, and `TypeDescription` allows their
/// enumeration.
///
/// A `TypeDescription` describes a base data format type, aggregation into
/// simple vector and matrix types, and an array length (if it's an array).
///
/// # For C++ Developers
///
/// The name was changed to not contain abbreviations (ergonomics). The original
/// name, [`TypeDesc`], is available behind a `type` alias when the
/// `cpp_api_names` feature is enabled.
///
/// [C++ Documentation](https://openimageio.readthedocs.io/en/latest/imageioapi.html#data-type-descriptions-typedesc)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct TypeDescription {
    pub base_type: Option<BaseType>,
    pub aggregate: Aggregate,
    pub vec_semantics: Option<VecSemantics>,
    pub array_len: Option<ArrayLen>,
}

/// Convenience type alias for developers familiar with the OpenImageIO C++ API.
///
/// # For C++ Developers
///
/// [C++ Documentation](https://openimageio.readthedocs.io/en/latest/imageioapi.html#data-type-descriptions-typedesc)
#[cfg(feature = "cpp_api_names")]
pub type TypeDesc = TypeDescription;

impl TypeDescription {
    pub fn is_array(&self) -> bool {
        matches!(self.array_len, Some(ArrayLen::Specific(_)))
    }

    pub fn size(&self) -> usize {
        let mut result = std::mem::MaybeUninit::<usize>::uninit();

        unsafe {
            oiio_TypeDesc_size(&self.into() as *const _ as _, &mut result as *mut _ as _);
            result.assume_init()
        }
    }

    pub fn element_size(&self) -> usize {
        let mut result = std::mem::MaybeUninit::<usize>::uninit();

        unsafe {
            oiio_TypeDesc_elementsize(&self.into() as *const _ as _, &mut result as *mut _ as _);
            result.assume_init()
        }
    }

    //pub fn scalar_type(&self) ->

    /// Is this a 2-vector aggregate (of the given type, [`BaseType::F32`] by
    /// default)?
    pub fn is_vec2(&self, base_type: Option<BaseType>) -> bool {
        let mut result = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_TypeDesc_is_vec2(
                &self.into() as *const _ as _,
                base_type.unwrap_or(BaseType::F32).into(),
                &mut result as *mut _ as _,
            );
            result.assume_init()
        }
    }

    /// Is this a 3-vector aggregate (of the given type, [`BaseType::F32`] by
    /// default)?
    pub fn is_vec3(&self, base_type: Option<BaseType>) -> bool {
        let mut result = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_TypeDesc_is_vec3(
                &self.into() as *const _ as _,
                base_type.unwrap_or(BaseType::F32).into(),
                &mut result as *mut _ as _,
            );
            result.assume_init()
        }
    }

    /// Is this a 4-vector aggregate (of the given type, [`BaseType::F32`] by
    /// default)?
    pub fn is_vec4(&self, base_type: Option<BaseType>) -> bool {
        let mut result = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_TypeDesc_is_vec4(
                &self.into() as *const _ as _,
                base_type.unwrap_or(BaseType::F32).into(),
                &mut result as *mut _ as _,
            );
            result.assume_init()
        }
    }
}

impl TryFrom<*const oiio_TypeDesc_t> for TypeDescription {
    type Error = ();

    //#[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn try_from(t: *const oiio_TypeDesc_t) -> Result<Self, ()> {
        match unsafe { t.as_ref() } {
            None => Err(()),
            Some(t) => Ok(t.into()),
        }
    }
}

impl From<&oiio_TypeDesc_t> for TypeDescription {
    fn from(t: &oiio_TypeDesc_t) -> Self {
        Self {
            base_type: match t.basetype {
                b if oiio_BASETYPE::oiio_BASETYPE_NONE.0 as u8 == b => None,
                b => b.try_into().ok(),
            },
            aggregate: t.aggregate.try_into().unwrap(),
            vec_semantics: match t.vecsemantics {
                b if oiio_VECSEMANTICS::oiio_VECSEMANTICS_NOXFORM.0 as u8 == b
                    || oiio_VECSEMANTICS::oiio_VECSEMANTICS_NOSEMANTICS.0 as u8 == b =>
                {
                    None
                }
                v => v.try_into().ok(),
            },
            array_len: match t.arraylen {
                l if 0 == l || l < -1 => None,
                -1 => Some(ArrayLen::Unspecific),
                l => Some(ArrayLen::Specific(NonZeroU32::new(l as _).unwrap())),
            },
        }
    }
}

impl From<&TypeDescription> for oiio_TypeDesc_t {
    fn from(t: &TypeDescription) -> oiio_TypeDesc_t {
        oiio_TypeDesc_t {
            basetype: match t.base_type {
                None => oiio_BASETYPE::oiio_BASETYPE_NONE.0 as _,
                Some(b) => b.into(),
            },
            aggregate: Into::<u8>::into(t.aggregate) as _,
            vecsemantics: match t.vec_semantics {
                None => oiio_VECSEMANTICS::oiio_VECSEMANTICS_NOSEMANTICS.0 as _,
                Some(v) => Into::<u8>::into(v) as _,
            },
            arraylen: match t.array_len {
                None => 0,
                Some(ArrayLen::Unspecific) => -1,
                Some(ArrayLen::Specific(l)) => l.get() as _,
            },
            reserved: 0,
        }
    }
}

impl From<TypeDescription> for oiio_TypeDesc_t {
    fn from(t: TypeDescription) -> Self {
        (&t).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_description() {
        let t = TypeDescription::default();

        let c_type = oiio_TypeDesc_t::from(&t);

        println!("C TypeDesc: {:?}", c_type);
    }
}
