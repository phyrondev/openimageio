use crate::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{ffi::c_int, num::NonZeroUsize};

/// Descrives the base data types that correspond (mostly) to the Rust
/// primitive/`std` types.
#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    TryFromPrimitive,
)]
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

/// Describes whether a [`TypeDesc`] is a simple scalar of one of the
/// [`BaseType`]s, or one of several simple aggregates.
///
/// Note that aggregates and arrays _enuare different. A `TypeDesc(F32, 3)`
/// is an array of three `f32`s, a `TypeDesc(F32, Aggregate::Vec3)` is a single
/// three-component vector comprised of `f32`s, and `TypeDesc(F32, 3,
/// Aggregate::Vec3)` is an array of three vectors, each of which is comprised
/// of three `f32`s.
#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    TryFromPrimitive,
)]
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
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    Hash,
    IntoPrimitive,
    TryFromPrimitive,
)]
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

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub enum ArrayLen {
    Specific(NonZeroUsize),
    #[default]
    Unspecific,
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TypeDesc {
    base_type: Option<BaseType>,
    aggregate: Aggregate,
    vec_semantics: Option<VecSemantics>,
    array_len: Option<ArrayLen>,
}

impl TryFrom<*const oiio_TypeDesc_t> for TypeDesc {
    type Error = ();

    fn try_from(t: *const oiio_TypeDesc_t) -> Result<Self, ()> {
        match unsafe { t.as_ref() } {
            None => return Err(()),
            Some(t) => Ok(Self {
                base_type: match t.basetype {
                    b if oiio_BASETYPE::oiio_BASETYPE_NONE.0 as u8 == b => None,
                    b => b.try_into().ok(),
                },
                aggregate: t.aggregate.try_into().unwrap(),
                vec_semantics: match t.vecsemantics {
                    b if oiio_VECSEMANTICS::oiio_VECSEMANTICS_NOXFORM.0
                        as u8
                        == b
                        || oiio_VECSEMANTICS::oiio_VECSEMANTICS_NOSEMANTICS.0
                            as u8
                            == b =>
                    {
                        None
                    }
                    v => v.try_into().ok(),
                },
                array_len: match t.arraylen {
                    l if 0 == l || l < -1 => None,
                    -1 => Some(ArrayLen::Unspecific),
                    l => l.try_into().ok().map(|l: c_int| {
                        ArrayLen::Specific(NonZeroUsize::new(l as _).unwrap())
                    }),
                },
            }),
        }
    }
}

impl From<&TypeDesc> for oiio_TypeDesc_t {
    fn from(t: &TypeDesc) -> oiio_TypeDesc_t {
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
