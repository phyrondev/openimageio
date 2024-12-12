use std::ffi::c_char;
use std::ffi::c_double;
use std::ffi::c_float;
use std::ffi::c_int;
use std::ffi::c_long;
use std::ffi::c_longlong;
use std::ffi::c_uchar;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_ushort;
use std::ffi::c_void;

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_Mode (pub c_uint);
impl oiio_Mode {
    pub const oiio_Mode_Closed: oiio_Mode = oiio_Mode(0);
    pub const oiio_Mode_Read: oiio_Mode = oiio_Mode(114);
    pub const oiio_Mode_Write: oiio_Mode = oiio_Mode(119);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_IBStorage (pub c_uint);
impl oiio_IBStorage {
    pub const oiio_IBStorage_UNINITIALIZED: oiio_IBStorage = oiio_IBStorage(0);
    pub const oiio_IBStorage_LOCALBUFFER: oiio_IBStorage = oiio_IBStorage(1);
    pub const oiio_IBStorage_APPBUFFER: oiio_IBStorage = oiio_IBStorage(2);
    pub const oiio_IBStorage_IMAGECACHE: oiio_IBStorage = oiio_IBStorage(3);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_WrapMode (pub c_uint);
impl oiio_WrapMode {
    pub const oiio_WrapMode_WrapDefault: oiio_WrapMode = oiio_WrapMode(0);
    pub const oiio_WrapMode_WrapBlack: oiio_WrapMode = oiio_WrapMode(1);
    pub const oiio_WrapMode_WrapClamp: oiio_WrapMode = oiio_WrapMode(2);
    pub const oiio_WrapMode_WrapPeriodic: oiio_WrapMode = oiio_WrapMode(3);
    pub const oiio_WrapMode_WrapMirror: oiio_WrapMode = oiio_WrapMode(4);
    pub const oiio_WrapMode__WrapLast: oiio_WrapMode = oiio_WrapMode(5);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_InitializePixels (pub c_int);
impl oiio_InitializePixels {
    pub const oiio_InitializePixels_No: oiio_InitializePixels = oiio_InitializePixels(0);
    pub const oiio_InitializePixels_Yes: oiio_InitializePixels = oiio_InitializePixels(1);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_TextAlignX (pub c_int);
impl oiio_TextAlignX {
    pub const oiio_TextAlignX_Left: oiio_TextAlignX = oiio_TextAlignX(0);
    pub const oiio_TextAlignX_Right: oiio_TextAlignX = oiio_TextAlignX(1);
    pub const oiio_TextAlignX_Center: oiio_TextAlignX = oiio_TextAlignX(2);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_TextAlignY (pub c_int);
impl oiio_TextAlignY {
    pub const oiio_TextAlignY_Baseline: oiio_TextAlignY = oiio_TextAlignY(0);
    pub const oiio_TextAlignY_Top: oiio_TextAlignY = oiio_TextAlignY(1);
    pub const oiio_TextAlignY_Bottom: oiio_TextAlignY = oiio_TextAlignY(2);
    pub const oiio_TextAlignY_Center: oiio_TextAlignY = oiio_TextAlignY(3);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_OpenMode (pub c_uint);
impl oiio_OpenMode {
    pub const oiio_OpenMode_Create: oiio_OpenMode = oiio_OpenMode(0);
    pub const oiio_OpenMode_AppendSubimage: oiio_OpenMode = oiio_OpenMode(1);
    pub const oiio_OpenMode_AppendMIPLevel: oiio_OpenMode = oiio_OpenMode(2);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_SerialFormat (pub c_uint);
impl oiio_SerialFormat {
    pub const oiio_SerialFormat_SerialText: oiio_SerialFormat = oiio_SerialFormat(0);
    pub const oiio_SerialFormat_SerialXML: oiio_SerialFormat = oiio_SerialFormat(1);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_SerialVerbose (pub c_uint);
impl oiio_SerialVerbose {
    pub const oiio_SerialVerbose_SerialBrief: oiio_SerialVerbose = oiio_SerialVerbose(0);
    pub const oiio_SerialVerbose_SerialDetailed: oiio_SerialVerbose = oiio_SerialVerbose(1);
    pub const oiio_SerialVerbose_SerialDetailedHuman: oiio_SerialVerbose = oiio_SerialVerbose(2);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_Interp (pub c_uint);
impl oiio_Interp {
    pub const oiio_Interp_INTERP_CONSTANT: oiio_Interp = oiio_Interp(0);
    pub const oiio_Interp_INTERP_PERPIECE: oiio_Interp = oiio_Interp(1);
    pub const oiio_Interp_INTERP_LINEAR: oiio_Interp = oiio_Interp(2);
    pub const oiio_Interp_INTERP_VERTEX: oiio_Interp = oiio_Interp(3);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_Wrap (pub u8);
impl oiio_Wrap {
    pub const oiio_Wrap_Default: oiio_Wrap = oiio_Wrap(0);
    pub const oiio_Wrap_Black: oiio_Wrap = oiio_Wrap(1);
    pub const oiio_Wrap_Clamp: oiio_Wrap = oiio_Wrap(2);
    pub const oiio_Wrap_Periodic: oiio_Wrap = oiio_Wrap(3);
    pub const oiio_Wrap_Mirror: oiio_Wrap = oiio_Wrap(4);
    pub const oiio_Wrap_PeriodicPow2: oiio_Wrap = oiio_Wrap(5);
    pub const oiio_Wrap_PeriodicSharedBorder: oiio_Wrap = oiio_Wrap(6);
    pub const oiio_Wrap_Last: oiio_Wrap = oiio_Wrap(7);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_MipMode (pub u8);
impl oiio_MipMode {
    pub const oiio_MipMode_Default: oiio_MipMode = oiio_MipMode(0);
    pub const oiio_MipMode_NoMIP: oiio_MipMode = oiio_MipMode(1);
    pub const oiio_MipMode_OneLevel: oiio_MipMode = oiio_MipMode(2);
    pub const oiio_MipMode_Trilinear: oiio_MipMode = oiio_MipMode(3);
    pub const oiio_MipMode_Aniso: oiio_MipMode = oiio_MipMode(4);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_InterpMode (pub u8);
impl oiio_InterpMode {
    pub const oiio_InterpMode_Closest: oiio_InterpMode = oiio_InterpMode(0);
    pub const oiio_InterpMode_Bilinear: oiio_InterpMode = oiio_InterpMode(1);
    pub const oiio_InterpMode_Bicubic: oiio_InterpMode = oiio_InterpMode(2);
    pub const oiio_InterpMode_SmartBicubic: oiio_InterpMode = oiio_InterpMode(3);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_AGGREGATE (pub c_uint);
impl oiio_AGGREGATE {
    pub const oiio_AGGREGATE_SCALAR: oiio_AGGREGATE = oiio_AGGREGATE(1);
    pub const oiio_AGGREGATE_VEC2: oiio_AGGREGATE = oiio_AGGREGATE(2);
    pub const oiio_AGGREGATE_VEC3: oiio_AGGREGATE = oiio_AGGREGATE(3);
    pub const oiio_AGGREGATE_VEC4: oiio_AGGREGATE = oiio_AGGREGATE(4);
    pub const oiio_AGGREGATE_MATRIX33: oiio_AGGREGATE = oiio_AGGREGATE(9);
    pub const oiio_AGGREGATE_MATRIX44: oiio_AGGREGATE = oiio_AGGREGATE(16);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_BASETYPE (pub c_uint);
impl oiio_BASETYPE {
    pub const oiio_BASETYPE_UNKNOWN: oiio_BASETYPE = oiio_BASETYPE(0);
    pub const oiio_BASETYPE_NONE: oiio_BASETYPE = oiio_BASETYPE(1);
    pub const oiio_BASETYPE_UINT8: oiio_BASETYPE = oiio_BASETYPE(2);
    pub const oiio_BASETYPE_UCHAR: oiio_BASETYPE = oiio_BASETYPE(2);
    pub const oiio_BASETYPE_INT8: oiio_BASETYPE = oiio_BASETYPE(3);
    pub const oiio_BASETYPE_CHAR: oiio_BASETYPE = oiio_BASETYPE(3);
    pub const oiio_BASETYPE_UINT16: oiio_BASETYPE = oiio_BASETYPE(4);
    pub const oiio_BASETYPE_USHORT: oiio_BASETYPE = oiio_BASETYPE(4);
    pub const oiio_BASETYPE_INT16: oiio_BASETYPE = oiio_BASETYPE(5);
    pub const oiio_BASETYPE_SHORT: oiio_BASETYPE = oiio_BASETYPE(5);
    pub const oiio_BASETYPE_UINT32: oiio_BASETYPE = oiio_BASETYPE(6);
    pub const oiio_BASETYPE_UINT: oiio_BASETYPE = oiio_BASETYPE(6);
    pub const oiio_BASETYPE_INT32: oiio_BASETYPE = oiio_BASETYPE(7);
    pub const oiio_BASETYPE_INT: oiio_BASETYPE = oiio_BASETYPE(7);
    pub const oiio_BASETYPE_UINT64: oiio_BASETYPE = oiio_BASETYPE(8);
    pub const oiio_BASETYPE_ULONGLONG: oiio_BASETYPE = oiio_BASETYPE(8);
    pub const oiio_BASETYPE_INT64: oiio_BASETYPE = oiio_BASETYPE(9);
    pub const oiio_BASETYPE_LONGLONG: oiio_BASETYPE = oiio_BASETYPE(9);
    pub const oiio_BASETYPE_HALF: oiio_BASETYPE = oiio_BASETYPE(10);
    pub const oiio_BASETYPE_FLOAT: oiio_BASETYPE = oiio_BASETYPE(11);
    pub const oiio_BASETYPE_DOUBLE: oiio_BASETYPE = oiio_BASETYPE(12);
    pub const oiio_BASETYPE_STRING: oiio_BASETYPE = oiio_BASETYPE(13);
    pub const oiio_BASETYPE_PTR: oiio_BASETYPE = oiio_BASETYPE(14);
    pub const oiio_BASETYPE_USTRINGHASH: oiio_BASETYPE = oiio_BASETYPE(15);
    pub const oiio_BASETYPE_LASTBASE: oiio_BASETYPE = oiio_BASETYPE(16);
}

#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct oiio_VECSEMANTICS (pub c_uint);
impl oiio_VECSEMANTICS {
    pub const oiio_VECSEMANTICS_NOXFORM: oiio_VECSEMANTICS = oiio_VECSEMANTICS(0);
    pub const oiio_VECSEMANTICS_NOSEMANTICS: oiio_VECSEMANTICS = oiio_VECSEMANTICS(0);
    pub const oiio_VECSEMANTICS_COLOR: oiio_VECSEMANTICS = oiio_VECSEMANTICS(1);
    pub const oiio_VECSEMANTICS_POINT: oiio_VECSEMANTICS = oiio_VECSEMANTICS(2);
    pub const oiio_VECSEMANTICS_VECTOR: oiio_VECSEMANTICS = oiio_VECSEMANTICS(3);
    pub const oiio_VECSEMANTICS_NORMAL: oiio_VECSEMANTICS = oiio_VECSEMANTICS(4);
    pub const oiio_VECSEMANTICS_TIMECODE: oiio_VECSEMANTICS = oiio_VECSEMANTICS(5);
    pub const oiio_VECSEMANTICS_KEYCODE: oiio_VECSEMANTICS = oiio_VECSEMANTICS(6);
    pub const oiio_VECSEMANTICS_RATIONAL: oiio_VECSEMANTICS = oiio_VECSEMANTICS(7);
    pub const oiio_VECSEMANTICS_BOX: oiio_VECSEMANTICS = oiio_VECSEMANTICS(8);
}

#[repr(C)]
pub struct oiio_ParamValueSpan_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_CspanF64_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_CspanF32_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_CspanU63_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_CspanU32_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_CspanI32_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_CspanU16_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_CspanU8_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_CspanString_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_String_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_StringView_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ColorConfig_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_DeepData_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_PointerVector_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_IOFile_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_Filter2D_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageBuf_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageBufSharedPtr_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_IteratorBase_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_Iterator_t {
    _unused: [u8; 0],
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct oiio_CompareResults_t {
    pub meanerror: c_double,
    pub rms_error: c_double,
    pub PSNR: c_double,
    pub maxerror: c_double,
    pub maxx: c_int,
    pub maxy: c_int,
    pub maxz: c_int,
    pub maxc: c_int,
    pub nwarn: u64,
    pub nfail: u64,
    pub error: bool,
}

#[repr(C)]
pub struct oiio_ImageCacheSharedPtr_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageCache_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageCachePerThreadInfo_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageCacheFile_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageCacheTile_t {
    _unused: [u8; 0],
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct oiio_ROI_t {
    pub xbegin: c_int,
    pub xend: c_int,
    pub ybegin: c_int,
    pub yend: c_int,
    pub zbegin: c_int,
    pub zend: c_int,
    pub chbegin: c_int,
    pub chend: c_int,
}

#[repr(C)]
pub struct oiio_ImageInputPtr_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageInput_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageOutputPtr_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageOutput_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_IOProxy_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ImageSpec_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ParamValue_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ParamValueList_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_TextureSystemSharedPtr_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_TextureSystem_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_Perthread_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_TextureOpt_v2_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_TextureHandle_t {
    _unused: [u8; 0],
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct oiio_TypeDesc_t {
    pub basetype: c_uchar,
    pub aggregate: c_uchar,
    pub vecsemantics: c_uchar,
    pub reserved: c_uchar,
    pub arraylen: c_int,
}

#[repr(C)]
pub struct oiio_TypeDescVector_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_ustring_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct oiio_oiio_M33fParam_t_t {
    _unused: [u8; 0],
}

unsafe extern "C" {

pub fn oiio_ParamValueSpan_ctor(list: *const oiio_ParamValueList_t, _result: *mut *mut oiio_ParamValueSpan_t) -> c_int;

pub fn oiio_ParamValueSpan_dtor(_this: *mut oiio_ParamValueSpan_t) -> c_int;

pub fn oiio_CspanF64_ctor(data: *mut c_double, size: c_ulong, _result: *mut *mut oiio_CspanF64_t) -> c_int;

pub fn oiio_CspanF64_dtor(_this: *mut oiio_CspanF64_t) -> c_int;

pub fn oiio_CspanF32_ctor(data: *mut c_float, size: c_ulong, _result: *mut *mut oiio_CspanF32_t) -> c_int;

pub fn oiio_CspanF32_dtor(_this: *mut oiio_CspanF32_t) -> c_int;

pub fn oiio_CspanU63_ctor(data: *mut c_ulong, size: c_ulong, _result: *mut *mut oiio_CspanU63_t) -> c_int;

pub fn oiio_CspanU63_dtor(_this: *mut oiio_CspanU63_t) -> c_int;

pub fn oiio_CspanU32_ctor(data: *mut c_uint, size: c_ulong, _result: *mut *mut oiio_CspanU32_t) -> c_int;

pub fn oiio_CspanU32_dtor(_this: *mut oiio_CspanU32_t) -> c_int;

pub fn oiio_CspanI32_ctor(data: *mut c_int, size: c_ulong, _result: *mut *mut oiio_CspanI32_t) -> c_int;

pub fn oiio_CspanI32_dtor(_this: *mut oiio_CspanI32_t) -> c_int;

pub fn oiio_CspanU16_ctor(data: *mut c_ushort, size: c_ulong, _result: *mut *mut oiio_CspanU16_t) -> c_int;

pub fn oiio_CspanU16_dtor(_this: *mut oiio_CspanU16_t) -> c_int;

pub fn oiio_CspanU8_ctor(data: *mut c_uchar, size: c_ulong, _result: *mut *mut oiio_CspanU8_t) -> c_int;

pub fn oiio_CspanU8_dtor(_this: *mut oiio_CspanU8_t) -> c_int;

pub fn oiio_CspanString_ctor(data: *mut oiio_String_t, size: c_ulong, _result: *mut *mut oiio_CspanString_t) -> c_int;

pub fn oiio_CspanString_dtor(_this: *mut oiio_CspanString_t) -> c_int;

pub fn oiio_String_c_str(_this: *const oiio_String_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_String_data(_this: *const oiio_String_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_String_empty(_this: *const oiio_String_t, _result: *mut bool) -> c_int;

pub fn oiio_String_size(_this: *const oiio_String_t, _result: *mut c_ulong) -> c_int;

pub fn oiio_String_length(_this: *const oiio_String_t, _result: *mut c_ulong) -> c_int;

pub fn oiio_String_ctor_default(_result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_String_ctor(s: *const c_char, count: c_ulong, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_String_dtor(_this: *mut oiio_String_t) -> c_int;

pub fn oiio_StringView_data(_this: *const oiio_StringView_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_StringView_size(_this: *const oiio_StringView_t, _result: *mut usize) -> c_int;

pub fn oiio_StringView_empty(_this: *const oiio_StringView_t, _result: *mut bool) -> c_int;

pub fn oiio_StringView_length(_this: *const oiio_StringView_t, _result: *mut usize) -> c_int;

pub fn oiio_StringView_default(_result: *mut *mut oiio_StringView_t) -> c_int;

pub fn oiio_StringView_ctor(s: *const c_char, count: c_ulong, _result: *mut *mut oiio_StringView_t) -> c_int;

pub fn oiio_StringView_dtor(_this: *mut oiio_StringView_t) -> c_int;

pub fn oiio_ColorConfig_has_error(_this: *const oiio_ColorConfig_t, _result: *mut bool) -> c_int;

pub fn oiio_ColorConfig_geterror(_this: *const oiio_ColorConfig_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ColorConfig_default(_result: *mut *mut oiio_ColorConfig_t) -> c_int;

pub fn oiio_ColorConfig_ctor(file_name: *mut oiio_StringView_t, _result: *mut *mut oiio_ColorConfig_t) -> c_int;

pub fn oiio_ColorConfig_dtor(_this: *mut oiio_ColorConfig_t) -> c_int;

pub fn oiio_DeepData_op_assign(_this: *mut oiio_DeepData_t, d: *const oiio_DeepData_t, _result: *mut *const oiio_DeepData_t) -> c_int;

pub fn oiio_DeepData_clear(_this: *mut oiio_DeepData_t) -> c_int;

pub fn oiio_DeepData_free(_this: *mut oiio_DeepData_t) -> c_int;

pub fn oiio_DeepData_init_with_imagespec(_this: *mut oiio_DeepData_t, spec: *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_DeepData_initialized(_this: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_DeepData_allocated(_this: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_DeepData_pixels(_this: *const oiio_DeepData_t, _result: *mut i64) -> c_int;

pub fn oiio_DeepData_channels(_this: *const oiio_DeepData_t, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_Z_channel(_this: *const oiio_DeepData_t, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_Zback_channel(_this: *const oiio_DeepData_t, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_A_channel(_this: *const oiio_DeepData_t, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_AR_channel(_this: *const oiio_DeepData_t, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_AG_channel(_this: *const oiio_DeepData_t, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_AB_channel(_this: *const oiio_DeepData_t, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_channeltype(_this: *const oiio_DeepData_t, c: c_int, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_DeepData_channelsize(_this: *const oiio_DeepData_t, c: c_int, _result: *mut usize) -> c_int;

pub fn oiio_DeepData_samplesize(_this: *const oiio_DeepData_t, _result: *mut usize) -> c_int;

pub fn oiio_DeepData_same_channeltypes(_this: *const oiio_DeepData_t, other: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_DeepData_samples(_this: *const oiio_DeepData_t, pixel: i64, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_set_samples(_this: *mut oiio_DeepData_t, pixel: i64, samps: c_int) -> c_int;

pub fn oiio_DeepData_set_capacity(_this: *mut oiio_DeepData_t, pixel: i64, samps: c_int) -> c_int;

pub fn oiio_DeepData_capacity(_this: *const oiio_DeepData_t, pixel: i64, _result: *mut c_int) -> c_int;

pub fn oiio_DeepData_insert_samples(_this: *mut oiio_DeepData_t, pixel: i64, samplepos: c_int, n: c_int) -> c_int;

pub fn oiio_DeepData_erase_samples(_this: *mut oiio_DeepData_t, pixel: i64, samplepos: c_int, n: c_int) -> c_int;

pub fn oiio_DeepData_deep_value(_this: *const oiio_DeepData_t, pixel: i64, channel: c_int, sample: c_int, _result: *mut c_float) -> c_int;

pub fn oiio_DeepData_deep_value_uint(_this: *const oiio_DeepData_t, pixel: i64, channel: c_int, sample: c_int, _result: *mut u32) -> c_int;

pub fn oiio_DeepData_set_deep_value_00(_this: *mut oiio_DeepData_t, pixel: i64, channel: c_int, sample: c_int, value: c_float) -> c_int;

pub fn oiio_DeepData_set_deep_value_01(_this: *mut oiio_DeepData_t, pixel: i64, channel: c_int, sample: c_int, value: u32) -> c_int;

pub fn oiio_DeepData_data_ptr_00(_this: *mut oiio_DeepData_t, pixel: i64, channel: c_int, sample: c_int, _result: *mut *mut c_void) -> c_int;

pub fn oiio_DeepData_data_ptr_01(_this: *const oiio_DeepData_t, pixel: i64, channel: c_int, sample: c_int, _result: *mut *const c_void) -> c_int;

pub fn oiio_DeepData_get_pointers(_this: *const oiio_DeepData_t, pointers: *mut oiio_PointerVector_t) -> c_int;

pub fn oiio_DeepData_copy_deep_sample(_this: *mut oiio_DeepData_t, pixel: i64, sample: c_int, src: *const oiio_DeepData_t, srcpixel: i64, srcsample: c_int, _result: *mut bool) -> c_int;

pub fn oiio_DeepData_copy_deep_pixel(_this: *mut oiio_DeepData_t, pixel: i64, src: *const oiio_DeepData_t, srcpixel: i64, _result: *mut bool) -> c_int;

pub fn oiio_DeepData_split(_this: *mut oiio_DeepData_t, pixel: i64, depth: c_float, _result: *mut bool) -> c_int;

pub fn oiio_DeepData_sort(_this: *mut oiio_DeepData_t, pixel: i64) -> c_int;

pub fn oiio_DeepData_merge_overlaps(_this: *mut oiio_DeepData_t, pixel: i64) -> c_int;

pub fn oiio_DeepData_merge_deep_pixels(_this: *mut oiio_DeepData_t, pixel: i64, src: *const oiio_DeepData_t, srcpixel: c_int) -> c_int;

pub fn oiio_DeepData_opaque_z(_this: *const oiio_DeepData_t, pixel: i64, _result: *mut c_float) -> c_int;

pub fn oiio_DeepData_occlusion_cull(_this: *mut oiio_DeepData_t, pixel: i64) -> c_int;

pub fn oiio_DeepData_default(_result: *mut *mut oiio_DeepData_t) -> c_int;

pub fn oiio_DeepData_ctor_01(spec: *const oiio_ImageSpec_t, _result: *mut *mut oiio_DeepData_t) -> c_int;

pub fn oiio_DeepData_dtor(_this: *mut oiio_DeepData_t) -> c_int;

pub fn oiio_PointerVector_dtor(_this: *mut oiio_PointerVector_t) -> c_int;

pub fn oiio_IOFile_ctor(file_name: *mut oiio_StringView_t, mode: oiio_Mode, _result: *mut *mut oiio_IOFile_t) -> c_int;

pub fn oiio_IOFile_dtor(_this: *mut oiio_IOFile_t) -> c_int;

pub fn oiio_Filter2D_create(filtername: *mut oiio_StringView_t, width: c_float, height: c_float, _result: *mut *mut oiio_Filter2D_t) -> c_int;

pub fn oiio_Filter2D_destroy(filt: *mut oiio_Filter2D_t) -> c_int;

pub fn oiio_Filter2D_width(_this: *const oiio_Filter2D_t, _result: *mut c_float) -> c_int;

pub fn oiio_Filter2D_height(_this: *const oiio_Filter2D_t, _result: *mut c_float) -> c_int;

pub fn oiio_Filter2D_dtor(_this: *mut oiio_Filter2D_t) -> c_int;

pub fn oiio_ImageBuf_clear(_this: *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_reset_00(_this: *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_reset_03(_this: *mut oiio_ImageBuf_t, spec: *const oiio_ImageSpec_t, zero: oiio_InitializePixels) -> c_int;

pub fn oiio_ImageBuf_reset_05(_this: *mut oiio_ImageBuf_t, spec: *const oiio_ImageSpec_t, buffer: *mut c_void, xstride: i64, ystride: i64, zstride: i64) -> c_int;

pub fn oiio_ImageBuf_make_writable(_this: *mut oiio_ImageBuf_t, keep_cache_type: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_set_write_tiles(_this: *mut oiio_ImageBuf_t, width: c_int, height: c_int, depth: c_int) -> c_int;

pub fn oiio_ImageBuf_set_write_ioproxy(_this: *mut oiio_ImageBuf_t, ioproxy: *mut oiio_IOProxy_t) -> c_int;

pub fn oiio_ImageBuf_op_assign_00(_this: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, _result: *mut *const oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_op_assign_01(_this: *mut oiio_ImageBuf_t, src: *mut oiio_ImageBuf_t, _result: *mut *const oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_copy_metadata(_this: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_copy_pixels(_this: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_copy_00(_this: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, format: oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_copy_01(_this: *const oiio_ImageBuf_t, format: oiio_TypeDesc_t, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_swap(_this: *mut oiio_ImageBuf_t, other: *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_getchannel(_this: *const oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, c: c_int, wrap: oiio_WrapMode, _result: *mut c_float) -> c_int;

pub fn oiio_ImageBuf_getpixel(_this: *const oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, pixel: *mut c_float, maxchannels: c_int, wrap: oiio_WrapMode) -> c_int;

pub fn oiio_ImageBuf_initialized(_this: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_storage(_this: *const oiio_ImageBuf_t, _result: *mut oiio_IBStorage) -> c_int;

pub fn oiio_ImageBuf_spec(_this: *const oiio_ImageBuf_t, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageBuf_specmod(_this: *mut oiio_ImageBuf_t, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageBuf_nativespec(_this: *const oiio_ImageBuf_t, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageBuf_has_thumbnail(_this: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_get_thumbnail(_this: *const oiio_ImageBuf_t, _result: *mut *mut oiio_ImageBufSharedPtr_t) -> c_int;

pub fn oiio_ImageBuf_clear_thumbnail(_this: *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_name(_this: *const oiio_ImageBuf_t, _result: *mut *mut oiio_StringView_t) -> c_int;

pub fn oiio_ImageBuf_file_format_name(_this: *const oiio_ImageBuf_t, _result: *mut *mut oiio_StringView_t) -> c_int;

pub fn oiio_ImageBuf_subimage(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_nsubimages(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_miplevel(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_nmiplevels(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_nchannels(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_xbegin(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_xend(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_ybegin(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_yend(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_zbegin(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_zend(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_xmin(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_xmax(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_ymin(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_ymax(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_zmin(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_zmax(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_orientation(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_oriented_width(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_oriented_height(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_oriented_x(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_oriented_y(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_oriented_full_width(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_oriented_full_height(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_oriented_full_x(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_oriented_full_y(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_roi(_this: *const oiio_ImageBuf_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageBuf_roi_full(_this: *const oiio_ImageBuf_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageBuf_contains_roi(_this: *const oiio_ImageBuf_t, roi: *const oiio_ROI_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_pixels_valid(_this: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_pixeltype(_this: *const oiio_ImageBuf_t, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_ImageBuf_localpixels_00(_this: *mut oiio_ImageBuf_t, _result: *mut *mut c_void) -> c_int;

pub fn oiio_ImageBuf_localpixels_01(_this: *const oiio_ImageBuf_t, _result: *mut *const c_void) -> c_int;

pub fn oiio_ImageBuf_pixel_stride(_this: *const oiio_ImageBuf_t, _result: *mut i64) -> c_int;

pub fn oiio_ImageBuf_scanline_stride(_this: *const oiio_ImageBuf_t, _result: *mut i64) -> c_int;

pub fn oiio_ImageBuf_z_stride(_this: *const oiio_ImageBuf_t, _result: *mut i64) -> c_int;

pub fn oiio_ImageBuf_contiguous(_this: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_cachedpixels(_this: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_imagecache(_this: *const oiio_ImageBuf_t, _result: *mut *mut oiio_ImageCacheSharedPtr_t) -> c_int;

pub fn oiio_ImageBuf_pixeladdr_00(_this: *const oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, ch: c_int, _result: *mut *const c_void) -> c_int;

pub fn oiio_ImageBuf_pixeladdr_01(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, ch: c_int, _result: *mut *mut c_void) -> c_int;

pub fn oiio_ImageBuf_pixelindex(_this: *const oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, check_range: bool, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_set_threads(_this: *const oiio_ImageBuf_t, n: c_int) -> c_int;

pub fn oiio_ImageBuf_threads(_this: *const oiio_ImageBuf_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_has_error(_this: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_geterror(_this: *const oiio_ImageBuf_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageBuf_deep(_this: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_deep_samples(_this: *const oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBuf_deep_pixel_ptr(_this: *const oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, _result: *mut *const c_void) -> c_int;

pub fn oiio_ImageBuf_deep_value(_this: *const oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, _result: *mut c_float) -> c_int;

pub fn oiio_ImageBuf_deep_value_uint(_this: *const oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, _result: *mut u32) -> c_int;

pub fn oiio_ImageBuf_deep_insert_samples(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, samplepos: c_int, nsamples: c_int) -> c_int;

pub fn oiio_ImageBuf_deep_erase_samples(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, samplepos: c_int, nsamples: c_int) -> c_int;

pub fn oiio_ImageBuf_copy_deep_pixel(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, src: *const oiio_ImageBuf_t, srcx: c_int, srcy: c_int, srcz: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_deepdata(_this: *mut oiio_ImageBuf_t, _result: *mut *mut oiio_DeepData_t) -> c_int;

pub fn oiio_ImageBuf_deepdata_const(_this: *const oiio_ImageBuf_t, _result: *mut *const oiio_DeepData_t) -> c_int;

pub fn oiio_ImageBuf_setpixel(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, pixel: *const c_float, maxchannels: c_int) -> c_int;

pub fn oiio_ImageBuf_set_deep_samples(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, nsamples: c_int) -> c_int;

pub fn oiio_ImageBuf_set_thumbnail(_this: *mut oiio_ImageBuf_t, thumb: *const oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_set_orientation(_this: *mut oiio_ImageBuf_t, orient: c_int) -> c_int;

pub fn oiio_ImageBuf_set_roi_full(_this: *mut oiio_ImageBuf_t, newroi: *const oiio_ROI_t) -> c_int;

pub fn oiio_ImageBuf_set_origin(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int) -> c_int;

pub fn oiio_ImageBuf_set_full(_this: *mut oiio_ImageBuf_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int) -> c_int;

pub fn oiio_ImageBuf_set_deep_value_00(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, value: c_float) -> c_int;

pub fn oiio_ImageBuf_set_deep_value_01(_this: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, value: u32) -> c_int;

pub fn oiio_ImageBuf_default(_result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_ctor_01(name: *mut oiio_StringView_t, subimage: c_int, miplevel: c_int, imagecache: *mut oiio_ImageCacheSharedPtr_t, config: *const oiio_ImageSpec_t, ioproxy: *mut oiio_IOProxy_t, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_ctor_02(name: *mut oiio_StringView_t, imagecache: *mut oiio_ImageCache_t, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_ctor_03(spec: *const oiio_ImageSpec_t, zero: oiio_InitializePixels, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_ctor_04(name: *mut oiio_StringView_t, spec: *const oiio_ImageSpec_t, zero: oiio_InitializePixels, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_ctor_05(spec: *const oiio_ImageSpec_t, buffer: *mut c_void, xstride: c_long, ystride: c_long, zstride: c_long, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_ctor_06(name: *mut oiio_StringView_t, spec: *const oiio_ImageSpec_t, buffer: *mut c_void, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_dtor(_this: *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_clear(_this: *mut oiio_ImageBufSharedPtr_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_reset_00(_this: *mut oiio_ImageBufSharedPtr_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_reset_03(_this: *mut oiio_ImageBufSharedPtr_t, spec: *const oiio_ImageSpec_t, zero: oiio_InitializePixels) -> c_int;

pub fn oiio_ImageBufSharedPtr_reset_05(_this: *mut oiio_ImageBufSharedPtr_t, spec: *const oiio_ImageSpec_t, buffer: *mut c_void, xstride: i64, ystride: i64, zstride: i64) -> c_int;

pub fn oiio_ImageBufSharedPtr_make_writable(_this: *mut oiio_ImageBufSharedPtr_t, keep_cache_type: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_write_tiles(_this: *mut oiio_ImageBufSharedPtr_t, width: c_int, height: c_int, depth: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_write_ioproxy(_this: *mut oiio_ImageBufSharedPtr_t, ioproxy: *mut oiio_IOProxy_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_op_assign_00(_this: *mut oiio_ImageBufSharedPtr_t, src: *const oiio_ImageBuf_t, _result: *mut *const oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_op_assign_01(_this: *mut oiio_ImageBufSharedPtr_t, src: *mut oiio_ImageBuf_t, _result: *mut *const oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_copy_metadata(_this: *mut oiio_ImageBufSharedPtr_t, src: *const oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_copy_pixels(_this: *mut oiio_ImageBufSharedPtr_t, src: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_copy_00(_this: *mut oiio_ImageBufSharedPtr_t, src: *const oiio_ImageBuf_t, format: oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_copy_01(_this: *const oiio_ImageBufSharedPtr_t, format: oiio_TypeDesc_t, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_swap(_this: *mut oiio_ImageBufSharedPtr_t, other: *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_getchannel(_this: *const oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, c: c_int, wrap: oiio_WrapMode, _result: *mut c_float) -> c_int;

pub fn oiio_ImageBufSharedPtr_getpixel(_this: *const oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, pixel: *mut c_float, maxchannels: c_int, wrap: oiio_WrapMode) -> c_int;

pub fn oiio_ImageBufSharedPtr_initialized(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_storage(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut oiio_IBStorage) -> c_int;

pub fn oiio_ImageBufSharedPtr_spec(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_specmod(_this: *mut oiio_ImageBufSharedPtr_t, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_nativespec(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_has_thumbnail(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_get_thumbnail(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut *mut oiio_ImageBufSharedPtr_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_clear_thumbnail(_this: *mut oiio_ImageBufSharedPtr_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_name(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut *mut oiio_StringView_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_file_format_name(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut *mut oiio_StringView_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_subimage(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_nsubimages(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_miplevel(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_nmiplevels(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_nchannels(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_xbegin(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_xend(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_ybegin(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_yend(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_zbegin(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_zend(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_xmin(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_xmax(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_ymin(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_ymax(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_zmin(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_zmax(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_orientation(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_oriented_width(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_oriented_height(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_oriented_x(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_oriented_y(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_oriented_full_width(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_oriented_full_height(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_oriented_full_x(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_oriented_full_y(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_roi(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_roi_full(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_contains_roi(_this: *const oiio_ImageBufSharedPtr_t, roi: *const oiio_ROI_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_pixels_valid(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_pixeltype(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_localpixels_00(_this: *mut oiio_ImageBufSharedPtr_t, _result: *mut *mut c_void) -> c_int;

pub fn oiio_ImageBufSharedPtr_localpixels_01(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut *const c_void) -> c_int;

pub fn oiio_ImageBufSharedPtr_pixel_stride(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut i64) -> c_int;

pub fn oiio_ImageBufSharedPtr_scanline_stride(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut i64) -> c_int;

pub fn oiio_ImageBufSharedPtr_z_stride(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut i64) -> c_int;

pub fn oiio_ImageBufSharedPtr_contiguous(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_cachedpixels(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_imagecache(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut *mut oiio_ImageCacheSharedPtr_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_pixeladdr_00(_this: *const oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, ch: c_int, _result: *mut *const c_void) -> c_int;

pub fn oiio_ImageBufSharedPtr_pixeladdr_01(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, ch: c_int, _result: *mut *mut c_void) -> c_int;

pub fn oiio_ImageBufSharedPtr_pixelindex(_this: *const oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, check_range: bool, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_threads(_this: *const oiio_ImageBufSharedPtr_t, n: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_threads(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_has_error(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_geterror(_this: *const oiio_ImageBufSharedPtr_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_deep(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_deep_samples(_this: *const oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, _result: *mut c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_deep_pixel_ptr(_this: *const oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, _result: *mut *const c_void) -> c_int;

pub fn oiio_ImageBufSharedPtr_deep_value(_this: *const oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, _result: *mut c_float) -> c_int;

pub fn oiio_ImageBufSharedPtr_deep_value_uint(_this: *const oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, _result: *mut u32) -> c_int;

pub fn oiio_ImageBufSharedPtr_deep_insert_samples(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, samplepos: c_int, nsamples: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_deep_erase_samples(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, samplepos: c_int, nsamples: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_copy_deep_pixel(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, src: *const oiio_ImageBuf_t, srcx: c_int, srcy: c_int, srcz: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufSharedPtr_deepdata(_this: *mut oiio_ImageBufSharedPtr_t, _result: *mut *mut oiio_DeepData_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_deepdata_const(_this: *const oiio_ImageBufSharedPtr_t, _result: *mut *const oiio_DeepData_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_setpixel(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, pixel: *const c_float, maxchannels: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_deep_samples(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, nsamples: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_thumbnail(_this: *mut oiio_ImageBufSharedPtr_t, thumb: *const oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_orientation(_this: *mut oiio_ImageBufSharedPtr_t, orient: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_roi_full(_this: *mut oiio_ImageBufSharedPtr_t, newroi: *const oiio_ROI_t) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_origin(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_full(_this: *mut oiio_ImageBufSharedPtr_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_deep_value_00(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, value: c_float) -> c_int;

pub fn oiio_ImageBufSharedPtr_set_deep_value_01(_this: *mut oiio_ImageBufSharedPtr_t, x: c_int, y: c_int, z: c_int, c: c_int, s: c_int, value: u32) -> c_int;

pub fn oiio_ImageBufSharedPtr_dtor(_this: *mut oiio_ImageBufSharedPtr_t) -> c_int;

pub fn oiio_IteratorBase_x(_this: *const oiio_IteratorBase_t, _result: *mut c_int) -> c_int;

pub fn oiio_IteratorBase_y(_this: *const oiio_IteratorBase_t, _result: *mut c_int) -> c_int;

pub fn oiio_IteratorBase_z(_this: *const oiio_IteratorBase_t, _result: *mut c_int) -> c_int;

pub fn oiio_IteratorBase_valid_00(_this: *const oiio_IteratorBase_t, _result: *mut bool) -> c_int;

pub fn oiio_IteratorBase_valid_01(_this: *const oiio_IteratorBase_t, x_: c_int, y_: c_int, z_: c_int, _result: *mut bool) -> c_int;

pub fn oiio_IteratorBase_exists_00(_this: *const oiio_IteratorBase_t, x_: c_int, y_: c_int, z_: c_int, _result: *mut bool) -> c_int;

pub fn oiio_IteratorBase_exists_01(_this: *const oiio_IteratorBase_t, _result: *mut bool) -> c_int;

pub fn oiio_IteratorBase_done(_this: *const oiio_IteratorBase_t, _result: *mut bool) -> c_int;

pub fn oiio_IteratorBase_deep_samples(_this: *const oiio_IteratorBase_t, _result: *mut c_int) -> c_int;

pub fn oiio_IteratorBase_wrap(_this: *const oiio_IteratorBase_t, _result: *mut oiio_WrapMode) -> c_int;

pub fn oiio_IteratorBase_pos(_this: *mut oiio_IteratorBase_t, x_: c_int, y_: c_int, z_: c_int) -> c_int;

pub fn oiio_IteratorBase_op_inc_00(_this: *mut oiio_IteratorBase_t) -> c_int;

pub fn oiio_IteratorBase_op_inc_01(_this: *mut oiio_IteratorBase_t, param00: c_int) -> c_int;

pub fn oiio_IteratorBase_range(_this: *const oiio_IteratorBase_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_IteratorBase_rerange(_this: *mut oiio_IteratorBase_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, wrap: oiio_WrapMode) -> c_int;

pub fn oiio_Iterator_op_assign(_this: *mut oiio_Iterator_t, param00: *const oiio_Iterator_t, _result: *mut *mut oiio_Iterator_t) -> c_int;

pub fn oiio_Iterator_rawptr(_this: *const oiio_Iterator_t, _result: *mut *mut c_void) -> c_int;

pub fn oiio_Iterator_set_deep_samples(_this: *mut oiio_Iterator_t, n: c_int) -> c_int;

pub fn oiio_Iterator_deep_value(_this: *const oiio_Iterator_t, c: c_int, s: c_int, _result: *mut c_float) -> c_int;

pub fn oiio_Iterator_deep_value_uint(_this: *const oiio_Iterator_t, c: c_int, s: c_int, _result: *mut u32) -> c_int;

pub fn oiio_Iterator_set_deep_value_00(_this: *mut oiio_Iterator_t, c: c_int, s: c_int, value: c_float) -> c_int;

pub fn oiio_Iterator_set_deep_value_01(_this: *mut oiio_Iterator_t, c: c_int, s: c_int, value: u32) -> c_int;

pub fn oiio_Iterator_x(_this: *const oiio_Iterator_t, _result: *mut c_int) -> c_int;

pub fn oiio_Iterator_y(_this: *const oiio_Iterator_t, _result: *mut c_int) -> c_int;

pub fn oiio_Iterator_z(_this: *const oiio_Iterator_t, _result: *mut c_int) -> c_int;

pub fn oiio_Iterator_valid_00(_this: *const oiio_Iterator_t, _result: *mut bool) -> c_int;

pub fn oiio_Iterator_valid_01(_this: *const oiio_Iterator_t, x_: c_int, y_: c_int, z_: c_int, _result: *mut bool) -> c_int;

pub fn oiio_Iterator_exists_00(_this: *const oiio_Iterator_t, x_: c_int, y_: c_int, z_: c_int, _result: *mut bool) -> c_int;

pub fn oiio_Iterator_exists_01(_this: *const oiio_Iterator_t, _result: *mut bool) -> c_int;

pub fn oiio_Iterator_done(_this: *const oiio_Iterator_t, _result: *mut bool) -> c_int;

pub fn oiio_Iterator_deep_samples(_this: *const oiio_Iterator_t, _result: *mut c_int) -> c_int;

pub fn oiio_Iterator_wrap(_this: *const oiio_Iterator_t, _result: *mut oiio_WrapMode) -> c_int;

pub fn oiio_Iterator_pos(_this: *mut oiio_Iterator_t, x_: c_int, y_: c_int, z_: c_int) -> c_int;

pub fn oiio_Iterator_op_inc_00(_this: *mut oiio_Iterator_t) -> c_int;

pub fn oiio_Iterator_op_inc_01(_this: *mut oiio_Iterator_t, param00: c_int) -> c_int;

pub fn oiio_Iterator_range(_this: *const oiio_Iterator_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_Iterator_rerange(_this: *mut oiio_Iterator_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, wrap: oiio_WrapMode) -> c_int;

pub fn oiio_Iterator_ctor_00(ib: *mut oiio_ImageBuf_t, wrap: oiio_WrapMode, _result: *mut *mut oiio_Iterator_t) -> c_int;

pub fn oiio_Iterator_ctor_01(ib: *mut oiio_ImageBuf_t, x: c_int, y: c_int, z: c_int, wrap: oiio_WrapMode, _result: *mut *mut oiio_Iterator_t) -> c_int;

pub fn oiio_Iterator_ctor_02(ib: *mut oiio_ImageBuf_t, roi: *const oiio_ROI_t, wrap: oiio_WrapMode, _result: *mut *mut oiio_Iterator_t) -> c_int;

pub fn oiio_Iterator_ctor_03(ib: *mut oiio_ImageBuf_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, wrap: oiio_WrapMode, _result: *mut *mut oiio_Iterator_t) -> c_int;

pub fn oiio_Iterator_dtor(_this: *mut oiio_Iterator_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_perthread_info(_this: *mut oiio_ImageCacheSharedPtr_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, _result: *mut *mut oiio_ImageCachePerThreadInfo_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_create_thread_info(_this: *mut oiio_ImageCacheSharedPtr_t, _result: *mut *mut oiio_ImageCachePerThreadInfo_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_destroy_thread_info(_this: *mut oiio_ImageCacheSharedPtr_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_image_handle(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, options: *const oiio_TextureOpt_v2_t, _result: *mut *mut oiio_ImageCacheFile_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_good(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_filename_from_handle(_this: *mut oiio_ImageCacheSharedPtr_t, handle: *mut oiio_ImageCacheFile_t, _result: *mut *mut oiio_ustring_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_resolve_filename(_this: *const oiio_ImageCacheSharedPtr_t, filename: *const oiio_String_t, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_image_info(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, dataname: *mut oiio_ustring_t, datatype: oiio_TypeDesc_t, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_image_info_from_handle(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, dataname: *mut oiio_ustring_t, datatype: oiio_TypeDesc_t, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_imagespec(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, spec: *mut oiio_ImageSpec_t, subimage: c_int, miplevel: c_int, native: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_imagespec_from_handle(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, spec: *mut oiio_ImageSpec_t, subimage: c_int, miplevel: c_int, native: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_imagespec(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, native: bool, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_imagespec_from_handle(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, native: bool, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_thumbnail(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, thumbnail: *mut oiio_ImageBuf_t, subimage: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_thumbnail_from_handle(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, thumbnail: *mut oiio_ImageBuf_t, subimage: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_pixels_00(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, result: *mut c_void, xstride: i64, ystride: i64, zstride: i64, cache_chbegin: c_int, cache_chend: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_pixels_01(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, result: *mut c_void, xstride: i64, ystride: i64, zstride: i64, cache_chbegin: c_int, cache_chend: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_pixels_02(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, format: oiio_TypeDesc_t, result: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_pixels_03(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, format: oiio_TypeDesc_t, result: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_invalidate_00(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, force: bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_invalidate_01(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, force: bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_invalidate_all(_this: *mut oiio_ImageCacheSharedPtr_t, force: bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_close(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_close_all(_this: *mut oiio_ImageCacheSharedPtr_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_tile(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, x: c_int, y: c_int, z: c_int, chbegin: c_int, chend: c_int, _result: *mut *mut oiio_ImageCacheTile_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_get_tile_from_handle(_this: *mut oiio_ImageCacheSharedPtr_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, x: c_int, y: c_int, z: c_int, chbegin: c_int, chend: c_int, _result: *mut *mut oiio_ImageCacheTile_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_release_tile(_this: *const oiio_ImageCacheSharedPtr_t, tile: *mut oiio_ImageCacheTile_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_tile_format(_this: *const oiio_ImageCacheSharedPtr_t, tile: *const oiio_ImageCacheTile_t, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_tile_roi(_this: *const oiio_ImageCacheSharedPtr_t, tile: *const oiio_ImageCacheTile_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_tile_pixels(_this: *const oiio_ImageCacheSharedPtr_t, tile: *mut oiio_ImageCacheTile_t, format: *mut oiio_TypeDesc_t, _result: *mut *const c_void) -> c_int;

pub fn oiio_ImageCacheSharedPtr_add_tile(_this: *mut oiio_ImageCacheSharedPtr_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, x: c_int, y: c_int, z: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, buffer: *const c_void, xstride: i64, ystride: i64, zstride: i64, copy: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_has_error(_this: *const oiio_ImageCacheSharedPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_geterror(_this: *const oiio_ImageCacheSharedPtr_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_getstats(_this: *const oiio_ImageCacheSharedPtr_t, level: c_int, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_reset_stats(_this: *mut oiio_ImageCacheSharedPtr_t) -> c_int;

pub fn oiio_ImageCacheSharedPtr_dtor(_this: *mut oiio_ImageCacheSharedPtr_t) -> c_int;

pub fn oiio_ImageCache_create(shared: bool, _result: *mut *mut oiio_ImageCacheSharedPtr_t) -> c_int;

pub fn oiio_ImageCache_destroy(cache: *mut oiio_ImageCacheSharedPtr_t, teardown: bool) -> c_int;

pub fn oiio_ImageCache_get_perthread_info(_this: *mut oiio_ImageCache_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, _result: *mut *mut oiio_ImageCachePerThreadInfo_t) -> c_int;

pub fn oiio_ImageCache_create_thread_info(_this: *mut oiio_ImageCache_t, _result: *mut *mut oiio_ImageCachePerThreadInfo_t) -> c_int;

pub fn oiio_ImageCache_destroy_thread_info(_this: *mut oiio_ImageCache_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t) -> c_int;

pub fn oiio_ImageCache_get_image_handle(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, options: *const oiio_TextureOpt_v2_t, _result: *mut *mut oiio_ImageCacheFile_t) -> c_int;

pub fn oiio_ImageCache_good(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_filename_from_handle(_this: *mut oiio_ImageCache_t, handle: *mut oiio_ImageCacheFile_t, _result: *mut *mut oiio_ustring_t) -> c_int;

pub fn oiio_ImageCache_resolve_filename(_this: *const oiio_ImageCache_t, filename: *const oiio_String_t, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageCache_get_image_info(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, dataname: *mut oiio_ustring_t, datatype: oiio_TypeDesc_t, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_get_image_info_from_handle(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, dataname: *mut oiio_ustring_t, datatype: oiio_TypeDesc_t, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_get_imagespec(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, spec: *mut oiio_ImageSpec_t, subimage: c_int, miplevel: c_int, native: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_get_imagespec_from_handle(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, spec: *mut oiio_ImageSpec_t, subimage: c_int, miplevel: c_int, native: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_imagespec(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, native: bool, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageCache_imagespec_from_handle(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, native: bool, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageCache_get_thumbnail(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, thumbnail: *mut oiio_ImageBuf_t, subimage: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_get_thumbnail_from_handle(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, thumbnail: *mut oiio_ImageBuf_t, subimage: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_get_pixels_00(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, result: *mut c_void, xstride: i64, ystride: i64, zstride: i64, cache_chbegin: c_int, cache_chend: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_get_pixels_01(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, result: *mut c_void, xstride: i64, ystride: i64, zstride: i64, cache_chbegin: c_int, cache_chend: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_get_pixels_02(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, format: oiio_TypeDesc_t, result: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_get_pixels_03(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, format: oiio_TypeDesc_t, result: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_invalidate_00(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, force: bool) -> c_int;

pub fn oiio_ImageCache_invalidate_01(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, force: bool) -> c_int;

pub fn oiio_ImageCache_invalidate_all(_this: *mut oiio_ImageCache_t, force: bool) -> c_int;

pub fn oiio_ImageCache_close(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t) -> c_int;

pub fn oiio_ImageCache_close_all(_this: *mut oiio_ImageCache_t) -> c_int;

pub fn oiio_ImageCache_get_tile(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, x: c_int, y: c_int, z: c_int, chbegin: c_int, chend: c_int, _result: *mut *mut oiio_ImageCacheTile_t) -> c_int;

pub fn oiio_ImageCache_get_tile_from_handle(_this: *mut oiio_ImageCache_t, file: *mut oiio_ImageCacheFile_t, thread_info: *mut oiio_ImageCachePerThreadInfo_t, subimage: c_int, miplevel: c_int, x: c_int, y: c_int, z: c_int, chbegin: c_int, chend: c_int, _result: *mut *mut oiio_ImageCacheTile_t) -> c_int;

pub fn oiio_ImageCache_release_tile(_this: *const oiio_ImageCache_t, tile: *mut oiio_ImageCacheTile_t) -> c_int;

pub fn oiio_ImageCache_tile_format(_this: *const oiio_ImageCache_t, tile: *const oiio_ImageCacheTile_t, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_ImageCache_tile_roi(_this: *const oiio_ImageCache_t, tile: *const oiio_ImageCacheTile_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageCache_tile_pixels(_this: *const oiio_ImageCache_t, tile: *mut oiio_ImageCacheTile_t, format: *mut oiio_TypeDesc_t, _result: *mut *const c_void) -> c_int;

pub fn oiio_ImageCache_add_tile(_this: *mut oiio_ImageCache_t, filename: *mut oiio_ustring_t, subimage: c_int, miplevel: c_int, x: c_int, y: c_int, z: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, buffer: *const c_void, xstride: i64, ystride: i64, zstride: i64, copy: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_has_error(_this: *const oiio_ImageCache_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_geterror(_this: *const oiio_ImageCache_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageCache_getstats(_this: *const oiio_ImageCache_t, level: c_int, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageCache_reset_stats(_this: *mut oiio_ImageCache_t) -> c_int;

pub fn oiio_ImageCache_dtor(_this: *mut oiio_ImageCache_t) -> c_int;

pub fn oiio_ROI_defined(_this: *const oiio_ROI_t, _result: *mut bool) -> c_int;

pub fn oiio_ROI_width(_this: *const oiio_ROI_t, _result: *mut c_int) -> c_int;

pub fn oiio_ROI_height(_this: *const oiio_ROI_t, _result: *mut c_int) -> c_int;

pub fn oiio_ROI_depth(_this: *const oiio_ROI_t, _result: *mut c_int) -> c_int;

pub fn oiio_ROI_nchannels(_this: *const oiio_ROI_t, _result: *mut c_int) -> c_int;

pub fn oiio_ROI_npixels(_this: *const oiio_ROI_t, _result: *mut u64) -> c_int;

pub fn oiio_ROI_All(_result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ROI_contains_region(_this: *const oiio_ROI_t, x: c_int, y: c_int, z: c_int, ch: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ROI_contains(_this: *const oiio_ROI_t, other: *const oiio_ROI_t, _result: *mut bool) -> c_int;

pub fn oiio_ROI_default(_result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ROI_with_dimensions(xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageInputPtr_format_name(_this: *const oiio_ImageInputPtr_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_ImageInputPtr_spec(_this: *const oiio_ImageInputPtr_t, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageInputPtr_spec_from_subimage(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageInputPtr_spec_dimensions(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageInputPtr_get_thumbnail(_this: *mut oiio_ImageInputPtr_t, thumb: *mut oiio_ImageBuf_t, subimage: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_close(_this: *mut oiio_ImageInputPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_current_subimage(_this: *const oiio_ImageInputPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageInputPtr_current_miplevel(_this: *const oiio_ImageInputPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageInputPtr_seek_subimage_00(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_scanline_00(_this: *mut oiio_ImageInputPtr_t, y: c_int, z: c_int, format: oiio_TypeDesc_t, data: *mut c_void, xstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_scanline_01(_this: *mut oiio_ImageInputPtr_t, y: c_int, z: c_int, data: *mut c_float, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_scanlines(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, ybegin: c_int, yend: c_int, z: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, data: *mut c_void, xstride: i64, ystride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_tile_00(_this: *mut oiio_ImageInputPtr_t, x: c_int, y: c_int, z: c_int, format: oiio_TypeDesc_t, data: *mut c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_tile_01(_this: *mut oiio_ImageInputPtr_t, x: c_int, y: c_int, z: c_int, data: *mut c_float, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_tiles(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, data: *mut c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_deep_scanlines(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, ybegin: c_int, yend: c_int, z: c_int, chbegin: c_int, chend: c_int, deepdata: *mut oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_deep_tiles(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, deepdata: *mut oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_deep_image(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, deepdata: *mut oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_scanline(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, y: c_int, z: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_scanlines_00(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, ybegin: c_int, yend: c_int, z: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_scanlines_01(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, ybegin: c_int, yend: c_int, z: c_int, chbegin: c_int, chend: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_tile(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, x: c_int, y: c_int, z: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_tiles_00(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_read_native_tiles_01(_this: *mut oiio_ImageInputPtr_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_set_ioproxy(_this: *mut oiio_ImageInputPtr_t, ioproxy: *mut oiio_IOProxy_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_has_error(_this: *const oiio_ImageInputPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_geterror(_this: *const oiio_ImageInputPtr_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageInputPtr_threads(_this: *mut oiio_ImageInputPtr_t, n: c_int) -> c_int;

pub fn oiio_ImageInputPtr_threads_const(_this: *const oiio_ImageInputPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageInputPtr_lock(_this: *const oiio_ImageInputPtr_t) -> c_int;

pub fn oiio_ImageInputPtr_unlock(_this: *const oiio_ImageInputPtr_t) -> c_int;

pub fn oiio_ImageInputPtr_try_lock(_this: *const oiio_ImageInputPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInputPtr_dtor(_this: *mut oiio_ImageInputPtr_t) -> c_int;

pub fn oiio_ImageInput_format_name(_this: *const oiio_ImageInput_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_ImageInput_spec(_this: *const oiio_ImageInput_t, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageInput_spec_from_subimage(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageInput_spec_dimensions(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageInput_get_thumbnail(_this: *mut oiio_ImageInput_t, thumb: *mut oiio_ImageBuf_t, subimage: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_close(_this: *mut oiio_ImageInput_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_current_subimage(_this: *const oiio_ImageInput_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageInput_current_miplevel(_this: *const oiio_ImageInput_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageInput_seek_subimage_00(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_scanline_00(_this: *mut oiio_ImageInput_t, y: c_int, z: c_int, format: oiio_TypeDesc_t, data: *mut c_void, xstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_scanline_01(_this: *mut oiio_ImageInput_t, y: c_int, z: c_int, data: *mut c_float, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_scanlines(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, ybegin: c_int, yend: c_int, z: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, data: *mut c_void, xstride: i64, ystride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_tile_00(_this: *mut oiio_ImageInput_t, x: c_int, y: c_int, z: c_int, format: oiio_TypeDesc_t, data: *mut c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_tile_01(_this: *mut oiio_ImageInput_t, x: c_int, y: c_int, z: c_int, data: *mut c_float, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_tiles(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, format: oiio_TypeDesc_t, data: *mut c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_deep_scanlines(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, ybegin: c_int, yend: c_int, z: c_int, chbegin: c_int, chend: c_int, deepdata: *mut oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_deep_tiles(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, deepdata: *mut oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_deep_image(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, deepdata: *mut oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_scanline(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, y: c_int, z: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_scanlines_00(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, ybegin: c_int, yend: c_int, z: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_scanlines_01(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, ybegin: c_int, yend: c_int, z: c_int, chbegin: c_int, chend: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_tile(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, x: c_int, y: c_int, z: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_tiles_00(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_read_native_tiles_01(_this: *mut oiio_ImageInput_t, subimage: c_int, miplevel: c_int, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, chbegin: c_int, chend: c_int, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_set_ioproxy(_this: *mut oiio_ImageInput_t, ioproxy: *mut oiio_IOProxy_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_has_error(_this: *const oiio_ImageInput_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_geterror(_this: *const oiio_ImageInput_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageInput_threads(_this: *mut oiio_ImageInput_t, n: c_int) -> c_int;

pub fn oiio_ImageInput_threads_const(_this: *const oiio_ImageInput_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageInput_lock(_this: *const oiio_ImageInput_t) -> c_int;

pub fn oiio_ImageInput_unlock(_this: *const oiio_ImageInput_t) -> c_int;

pub fn oiio_ImageInput_try_lock(_this: *const oiio_ImageInput_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_dtor(_this: *mut oiio_ImageInput_t) -> c_int;

pub fn oiio_ImageOutputPtr_format_name(_this: *const oiio_ImageOutputPtr_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_ImageOutputPtr_spec(_this: *const oiio_ImageOutputPtr_t, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageOutputPtr_close(_this: *mut oiio_ImageOutputPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_write_scanline(_this: *mut oiio_ImageOutputPtr_t, y: c_int, z: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_write_scanlines(_this: *mut oiio_ImageOutputPtr_t, ybegin: c_int, yend: c_int, z: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, ystride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_write_tile(_this: *mut oiio_ImageOutputPtr_t, x: c_int, y: c_int, z: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_write_tiles(_this: *mut oiio_ImageOutputPtr_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_write_rectangle(_this: *mut oiio_ImageOutputPtr_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_write_deep_scanlines(_this: *mut oiio_ImageOutputPtr_t, ybegin: c_int, yend: c_int, z: c_int, deepdata: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_write_deep_tiles(_this: *mut oiio_ImageOutputPtr_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, deepdata: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_write_deep_image(_this: *mut oiio_ImageOutputPtr_t, deepdata: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_set_thumbnail(_this: *mut oiio_ImageOutputPtr_t, thumb: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_copy_image(_this: *mut oiio_ImageOutputPtr_t, in_: *mut oiio_ImageInput_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_set_ioproxy(_this: *mut oiio_ImageOutputPtr_t, ioproxy: *mut oiio_IOProxy_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_has_error(_this: *const oiio_ImageOutputPtr_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutputPtr_geterror(_this: *const oiio_ImageOutputPtr_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageOutputPtr_setthreads(_this: *mut oiio_ImageOutputPtr_t, n: c_int) -> c_int;

pub fn oiio_ImageOutputPtr_getthreads(_this: *const oiio_ImageOutputPtr_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageOutputPtr_dtor(_this: *mut oiio_ImageOutputPtr_t) -> c_int;

pub fn oiio_ImageOutput_format_name(_this: *const oiio_ImageOutput_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_ImageOutput_spec(_this: *const oiio_ImageOutput_t, _result: *mut *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageOutput_close(_this: *mut oiio_ImageOutput_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_write_scanline(_this: *mut oiio_ImageOutput_t, y: c_int, z: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_write_scanlines(_this: *mut oiio_ImageOutput_t, ybegin: c_int, yend: c_int, z: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, ystride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_write_tile(_this: *mut oiio_ImageOutput_t, x: c_int, y: c_int, z: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_write_tiles(_this: *mut oiio_ImageOutput_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_write_rectangle(_this: *mut oiio_ImageOutput_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, format: oiio_TypeDesc_t, data: *const c_void, xstride: i64, ystride: i64, zstride: i64, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_write_deep_scanlines(_this: *mut oiio_ImageOutput_t, ybegin: c_int, yend: c_int, z: c_int, deepdata: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_write_deep_tiles(_this: *mut oiio_ImageOutput_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, deepdata: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_write_deep_image(_this: *mut oiio_ImageOutput_t, deepdata: *const oiio_DeepData_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_set_thumbnail(_this: *mut oiio_ImageOutput_t, thumb: *const oiio_ImageBuf_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_copy_image(_this: *mut oiio_ImageOutput_t, in_: *mut oiio_ImageInput_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_set_ioproxy(_this: *mut oiio_ImageOutput_t, ioproxy: *mut oiio_IOProxy_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_has_error(_this: *const oiio_ImageOutput_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_geterror(_this: *const oiio_ImageOutput_t, clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageOutput_setthreads(_this: *mut oiio_ImageOutput_t, n: c_int) -> c_int;

pub fn oiio_ImageOutput_getthreads(_this: *const oiio_ImageOutput_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageOutput_dtor(_this: *mut oiio_ImageOutput_t) -> c_int;

pub fn oiio_ImageSpec_set_format(_this: *mut oiio_ImageSpec_t, fmt: oiio_TypeDesc_t) -> c_int;

pub fn oiio_ImageSpec_default_channel_names(_this: *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageSpec_channel_bytes_00(_this: *const oiio_ImageSpec_t, _result: *mut usize) -> c_int;

pub fn oiio_ImageSpec_channel_bytes_01(_this: *const oiio_ImageSpec_t, chan: c_int, native: bool, _result: *mut usize) -> c_int;

pub fn oiio_ImageSpec_pixel_bytes_00(_this: *const oiio_ImageSpec_t, native: bool, _result: *mut usize) -> c_int;

pub fn oiio_ImageSpec_pixel_bytes_01(_this: *const oiio_ImageSpec_t, chbegin: c_int, chend: c_int, native: bool, _result: *mut usize) -> c_int;

pub fn oiio_ImageSpec_scanline_bytes(_this: *const oiio_ImageSpec_t, native: bool, _result: *mut u64) -> c_int;

pub fn oiio_ImageSpec_tile_pixels(_this: *const oiio_ImageSpec_t, _result: *mut u64) -> c_int;

pub fn oiio_ImageSpec_tile_bytes(_this: *const oiio_ImageSpec_t, native: bool, _result: *mut u64) -> c_int;

pub fn oiio_ImageSpec_image_pixels(_this: *const oiio_ImageSpec_t, _result: *mut u64) -> c_int;

pub fn oiio_ImageSpec_image_bytes(_this: *const oiio_ImageSpec_t, native: bool, _result: *mut u64) -> c_int;

pub fn oiio_ImageSpec_size_t_safe(_this: *const oiio_ImageSpec_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageSpec_auto_stride_00(xstride: *mut i64, ystride: *mut i64, zstride: *mut i64, channelsize: i64, nchannels: c_int, width: c_int, height: c_int) -> c_int;

pub fn oiio_ImageSpec_auto_stride_01(xstride: *mut i64, ystride: *mut i64, zstride: *mut i64, format: oiio_TypeDesc_t, nchannels: c_int, width: c_int, height: c_int) -> c_int;

pub fn oiio_ImageSpec_auto_stride_02(xstride: *mut i64, format: oiio_TypeDesc_t, nchannels: c_int) -> c_int;

pub fn oiio_ImageSpec_metadata_val(p: *const oiio_ParamValue_t, human: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageSpec_serialize(_this: *const oiio_ImageSpec_t, format: oiio_SerialFormat, verbose: oiio_SerialVerbose, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageSpec_to_xml(_this: *const oiio_ImageSpec_t, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageSpec_from_xml(_this: *mut oiio_ImageSpec_t, xml: *const c_char) -> c_int;

pub fn oiio_ImageSpec_valid_tile_range(_this: *mut oiio_ImageSpec_t, xbegin: c_int, xend: c_int, ybegin: c_int, yend: c_int, zbegin: c_int, zend: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageSpec_channelformat(_this: *const oiio_ImageSpec_t, chan: c_int, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_ImageSpec_get_channelformats(_this: *const oiio_ImageSpec_t, formats: *mut oiio_TypeDescVector_t) -> c_int;

pub fn oiio_ImageSpec_roi(_this: *const oiio_ImageSpec_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageSpec_roi_full(_this: *const oiio_ImageSpec_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageSpec_set_roi(_this: *mut oiio_ImageSpec_t, r: *const oiio_ROI_t) -> c_int;

pub fn oiio_ImageSpec_set_roi_full(_this: *mut oiio_ImageSpec_t, r: *const oiio_ROI_t) -> c_int;

pub fn oiio_ImageSpec_set_colorspace(_this: *mut oiio_ImageSpec_t, name: *mut oiio_StringView_t) -> c_int;

pub fn oiio_ImageSpec_copy_dimensions(_this: *mut oiio_ImageSpec_t, other: *const oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageSpec_undefined(_this: *const oiio_ImageSpec_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageSpec_new(format: oiio_TypeDesc_t, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageSpec_with_dimensions(xres: c_int, yres: c_int, nchans: c_int, fmt: oiio_TypeDesc_t, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageSpec_with_region(roi: *const oiio_ROI_t, fmt: oiio_TypeDesc_t, _result: *mut *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ImageSpec_dtor(_this: *mut oiio_ImageSpec_t) -> c_int;

pub fn oiio_ParamValue_op_assign_00(_this: *mut oiio_ParamValue_t, p: *const oiio_ParamValue_t, _result: *mut *const oiio_ParamValue_t) -> c_int;

pub fn oiio_ParamValue_op_assign_01(_this: *mut oiio_ParamValue_t, p: *mut oiio_ParamValue_t, _result: *mut *const oiio_ParamValue_t) -> c_int;

pub fn oiio_ParamValue_name(_this: *const oiio_ParamValue_t, _result: *mut *const oiio_ustring_t) -> c_int;

pub fn oiio_ParamValue_uname(_this: *const oiio_ParamValue_t, _result: *mut *const oiio_ustring_t) -> c_int;

pub fn oiio_ParamValue_type(_this: *const oiio_ParamValue_t, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_ParamValue_nvalues(_this: *const oiio_ParamValue_t, _result: *mut c_int) -> c_int;

pub fn oiio_ParamValue_data(_this: *const oiio_ParamValue_t, _result: *mut *const c_void) -> c_int;

pub fn oiio_ParamValue_datasize(_this: *const oiio_ParamValue_t, _result: *mut c_int) -> c_int;

pub fn oiio_ParamValue_interp_00(_this: *const oiio_ParamValue_t, _result: *mut oiio_Interp) -> c_int;

pub fn oiio_ParamValue_interp_01(_this: *mut oiio_ParamValue_t, i: oiio_Interp) -> c_int;

pub fn oiio_ParamValue_is_nonlocal(_this: *const oiio_ParamValue_t, _result: *mut bool) -> c_int;

pub fn oiio_ParamValue_get_int(_this: *const oiio_ParamValue_t, defaultval: c_int, _result: *mut c_int) -> c_int;

pub fn oiio_ParamValue_get_int_indexed(_this: *const oiio_ParamValue_t, index: c_int, defaultval: c_int, _result: *mut c_int) -> c_int;

pub fn oiio_ParamValue_get_float(_this: *const oiio_ParamValue_t, defaultval: c_float, _result: *mut c_float) -> c_int;

pub fn oiio_ParamValue_get_float_indexed(_this: *const oiio_ParamValue_t, index: c_int, defaultval: c_float, _result: *mut c_float) -> c_int;

pub fn oiio_ParamValue_get_string(_this: *const oiio_ParamValue_t, maxsize: c_int, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ParamValue_get_string_indexed(_this: *const oiio_ParamValue_t, index: c_int, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ParamValue_get_ustring(_this: *const oiio_ParamValue_t, maxsize: c_int, _result: *mut *mut oiio_ustring_t) -> c_int;

pub fn oiio_ParamValue_get_ustring_indexed(_this: *const oiio_ParamValue_t, index: c_int, _result: *mut *mut oiio_ustring_t) -> c_int;

pub fn oiio_ParamValue_default(_result: *mut *mut oiio_ParamValue_t) -> c_int;

pub fn oiio_ParamValue_dtor(_this: *mut oiio_ParamValue_t) -> c_int;

pub fn oiio_ParamValueList_grow(_this: *mut oiio_ParamValueList_t, _result: *mut *mut oiio_ParamValue_t) -> c_int;

pub fn oiio_ParamValueList_add_or_replace(_this: *mut oiio_ParamValueList_t, pv: *const oiio_ParamValue_t, casesensitive: bool) -> c_int;

pub fn oiio_ParamValueList_sort(_this: *mut oiio_ParamValueList_t, casesensitive: bool) -> c_int;

pub fn oiio_ParamValueList_merge(_this: *mut oiio_ParamValueList_t, other: *const oiio_ParamValueList_t, override_: bool) -> c_int;

pub fn oiio_ParamValueList_free(_this: *mut oiio_ParamValueList_t) -> c_int;

pub fn oiio_ParamValueList_default(_result: *mut *mut oiio_ParamValueList_t) -> c_int;

pub fn oiio_ParamValueList_dtor(_this: *mut oiio_ParamValueList_t) -> c_int;

pub fn oiio_TextureSystemSharedPtr_dtor(_this: *mut oiio_TextureSystemSharedPtr_t) -> c_int;

pub fn oiio_TextureSystem_create(shared: bool, imagecache: *mut oiio_ImageCacheSharedPtr_t, _result: *mut *mut oiio_TextureSystemSharedPtr_t) -> c_int;

pub fn oiio_TextureSystem_destroy(ts: *mut oiio_TextureSystemSharedPtr_t, teardown_imagecache: bool) -> c_int;

pub fn oiio_TextureSystem_dtor(_this: *mut oiio_TextureSystem_t) -> c_int;

pub fn oiio_TextureOpt_v2_default(_result: *mut *mut oiio_TextureOpt_v2_t) -> c_int;

pub fn oiio_TextureOpt_v2_dtor(_this: *mut oiio_TextureOpt_v2_t) -> c_int;

pub fn oiio_TypeDesc_c_str(_this: *const oiio_TypeDesc_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_TypeDesc_numelements(_this: *const oiio_TypeDesc_t, _result: *mut usize) -> c_int;

pub fn oiio_TypeDesc_basevalues(_this: *const oiio_TypeDesc_t, _result: *mut usize) -> c_int;

pub fn oiio_TypeDesc_is_array(_this: *const oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_unsized_array(_this: *const oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_sized_array(_this: *const oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_size(_this: *const oiio_TypeDesc_t, _result: *mut usize) -> c_int;

pub fn oiio_TypeDesc_elementtype(_this: *const oiio_TypeDesc_t, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDesc_elementsize(_this: *const oiio_TypeDesc_t, _result: *mut usize) -> c_int;

pub fn oiio_TypeDesc_scalartype(_this: *const oiio_TypeDesc_t, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDesc_basesize(_this: *const oiio_TypeDesc_t, _result: *mut usize) -> c_int;

pub fn oiio_TypeDesc_is_floating_point(_this: *const oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_signed(_this: *const oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_unknown(_this: *const oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_op_eq(_this: *const oiio_TypeDesc_t, t: *const oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_equivalent(_this: *const oiio_TypeDesc_t, b: *const oiio_TypeDesc_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_vec2(_this: *const oiio_TypeDesc_t, b: oiio_BASETYPE, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_vec3(_this: *const oiio_TypeDesc_t, b: oiio_BASETYPE, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_vec4(_this: *const oiio_TypeDesc_t, b: oiio_BASETYPE, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_box2(_this: *const oiio_TypeDesc_t, b: oiio_BASETYPE, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_is_box3(_this: *const oiio_TypeDesc_t, b: oiio_BASETYPE, _result: *mut bool) -> c_int;

pub fn oiio_TypeDesc_unarray(_this: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDesc_ctor_00(btype: oiio_BASETYPE, agg: oiio_AGGREGATE, semantics: oiio_VECSEMANTICS, arraylen: c_int, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDesc_ctor_01(btype: oiio_BASETYPE, arraylen: c_int, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDesc_ctor_02(btype: oiio_BASETYPE, agg: oiio_AGGREGATE, arraylen: c_int, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDescVector_data(_this: *mut oiio_TypeDescVector_t, _result: *mut *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDescVector_data_const(_this: *const oiio_TypeDescVector_t, _result: *mut *const oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDescVector_empty(_this: *const oiio_TypeDescVector_t, _result: *mut bool) -> c_int;

pub fn oiio_TypeDescVector_size(_this: *const oiio_TypeDescVector_t, _result: *mut usize) -> c_int;

pub fn oiio_TypeDescVector_max_size(_this: *const oiio_TypeDescVector_t, _result: *mut usize) -> c_int;

pub fn oiio_TypeDescVector_capacity(_this: *const oiio_TypeDescVector_t, _result: *mut usize) -> c_int;

pub fn oiio_TypeDescVector_clear(_this: *mut oiio_TypeDescVector_t) -> c_int;

pub fn oiio_TypeDescVector_pop_back(_this: *mut oiio_TypeDescVector_t) -> c_int;

pub fn oiio_TypeDescVector_op_index(_this: *const oiio_TypeDescVector_t, __n: usize, _result: *mut *const oiio_TypeDesc_t) -> c_int;

pub fn oiio_TypeDescVector_default(_result: *mut *mut oiio_TypeDescVector_t) -> c_int;

pub fn oiio_TypeDescVector_dtor(_this: *mut oiio_TypeDescVector_t) -> c_int;

pub fn oiio_ustring_c_str(_this: *const oiio_ustring_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_ustring_data(_this: *const oiio_ustring_t, _result: *mut *const c_char) -> c_int;

pub fn oiio_ustring_hash(_this: *const oiio_ustring_t, _result: *mut u64) -> c_int;

pub fn oiio_ustring_size(_this: *const oiio_ustring_t, _result: *mut usize) -> c_int;

pub fn oiio_ustring_empty(_this: *const oiio_ustring_t, _result: *mut bool) -> c_int;

pub fn oiio_ustring_op_lt(_this: *const oiio_ustring_t, x: *const oiio_ustring_t, _result: *mut bool) -> c_int;

pub fn oiio_ustring_default(_result: *mut *mut oiio_ustring_t) -> c_int;

pub fn oiio_ustring_new(str: *const c_char, _result: *mut *mut oiio_ustring_t) -> c_int;

pub fn oiio_ustring_new_from_parts(str: *const c_char, len: c_ulong, _result: *mut *mut oiio_ustring_t) -> c_int;

pub fn oiio_ustring_dtor(_this: *mut oiio_ustring_t) -> c_int;

pub fn oiio_oiio_M33fParam_t_data(_this: *const oiio_oiio_M33fParam_t_t, _result: *mut *const c_float) -> c_int;

pub fn oiio_oiio_M33fParam_t_dtor(_this: *mut oiio_oiio_M33fParam_t_t) -> c_int;

pub fn oiio_openimageio_version(_result: *mut c_int) -> c_int;

pub fn oiio_has_error(_result: *mut bool) -> c_int;

pub fn oiio_geterror(clear: bool, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_DeepData_all_channeltypes(_this: *const oiio_DeepData_t, types: *mut *const oiio_TypeDesc_t, num_types: *mut c_longlong) -> c_int;

pub fn oiio_DeepData_all_samples(_this: *const oiio_DeepData_t, samples: *mut *const c_uint, num_types: *mut c_longlong) -> c_int;

pub fn oiio_DeepData_all_data(_this: *const oiio_DeepData_t, data: *mut *const c_char, num_types: *mut c_longlong) -> c_int;

pub fn oiio_DeepData_channelname(dd: *const oiio_DeepData_t, c: c_int, _result: *mut *const c_char) -> c_int;

pub fn oiio_DeepData_set_all_samples(dd: *mut oiio_DeepData_t, samples: *const c_uint, num: c_ulong) -> c_int;

pub fn oiio_ImageBuf_WrapMode_from_string(name: *const c_char, _result: *mut oiio_WrapMode) -> c_int;

pub fn oiio_ImageBuf_write(buf: *const oiio_ImageBuf_t, file_name: *const oiio_StringView_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_write_with_spec(buf: *const oiio_ImageBuf_t, file_name: *const oiio_StringView_t, type_desc: oiio_TypeDesc_t, file_format: *const oiio_StringView_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_get_pixels(buf: *const oiio_ImageBuf_t, roi: oiio_ROI_t, base_type: oiio_BASETYPE, result: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_set_pixels_f32(buf: *mut oiio_ImageBuf_t, roi: oiio_ROI_t, pixels: *mut oiio_CspanF32_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_set_pixels_f64(buf: *mut oiio_ImageBuf_t, roi: oiio_ROI_t, pixels: *mut oiio_CspanF64_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_set_pixels_u32(buf: *mut oiio_ImageBuf_t, roi: oiio_ROI_t, pixels: *mut oiio_CspanU32_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_set_pixels_u16(buf: *mut oiio_ImageBuf_t, roi: oiio_ROI_t, pixels: *mut oiio_CspanU16_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_set_pixels_u8(buf: *mut oiio_ImageBuf_t, roi: oiio_ROI_t, pixels: *mut oiio_CspanU8_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageBuf_expand_roi_full(buf: *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBuf_from_dimensions(width: c_int, height: c_int, nchannels: c_int, format: oiio_TypeDesc_t, color_space: *mut oiio_StringView_t, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufAlgo_zero(dst: *mut oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_checker(dst: *mut oiio_ImageBuf_t, width: c_int, height: c_int, depth: c_int, color1: *mut oiio_CspanF32_t, color2: *mut oiio_CspanF32_t, xoffset: c_int, yoffset: c_int, zoffset: c_int, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_fill(dst: *mut oiio_ImageBuf_t, values: *mut oiio_CspanF32_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_fill_vertical(dst: *mut oiio_ImageBuf_t, top: *mut oiio_CspanF32_t, bottom: *mut oiio_CspanF32_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_fill_corners(dst: *mut oiio_ImageBuf_t, topleft: *mut oiio_CspanF32_t, topright: *mut oiio_CspanF32_t, bottomleft: *mut oiio_CspanF32_t, bottomright: *mut oiio_CspanF32_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_noise(dst: *mut oiio_ImageBuf_t, noisetype: *mut oiio_StringView_t, A: c_float, B: c_float, mono: bool, seed: c_int, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_over(dst: *mut oiio_ImageBuf_t, A: *const oiio_ImageBuf_t, B: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_from_over(A: *const oiio_ImageBuf_t, B: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufAlgo_zover(dst: *mut oiio_ImageBuf_t, A: *const oiio_ImageBuf_t, B: *const oiio_ImageBuf_t, z_zeroisinf: bool, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_reorient(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_rotate(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, angle: c_float, filter: *mut oiio_Filter2D_t, recompute_roi: bool, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_rotate_around(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, angle: c_float, center_x: c_float, center_y: c_float, filter: *mut oiio_Filter2D_t, recompute_roi: bool, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_rotate90(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_rotate180(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_rotate270(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_compare(A: *const oiio_ImageBuf_t, B: *const oiio_ImageBuf_t, failthresh: c_float, warnthresh: c_float, roi: oiio_ROI_t, nthreads: c_int, _result: *mut oiio_CompareResults_t) -> c_int;

pub fn oiio_ImageBufAlgo_render_text(dst: *mut oiio_ImageBuf_t, x: c_int, y: c_int, text: *mut oiio_StringView_t, fontsize: c_int, fontname: *mut oiio_StringView_t, textcolor: *mut oiio_CspanF32_t, alignx: oiio_TextAlignX, aligny: oiio_TextAlignY, shadow: c_int, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_colorconvert(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, fromspace: *mut oiio_StringView_t, tospace: *mut oiio_StringView_t, unpremult: bool, context_key: *mut oiio_StringView_t, context_value: *mut oiio_StringView_t, colorconfig: *const oiio_ColorConfig_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_resample(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, interpolate: bool, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_resize(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, options: *mut oiio_ParamValueSpan_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_warp(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, M: *mut oiio_oiio_M33fParam_t_t, filter: *const oiio_Filter2D_t, recompute_roi: bool, wrap: oiio_WrapMode, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_st_warp(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, stbuf: *const oiio_ImageBuf_t, filter: *const oiio_Filter2D_t, chan_s: c_int, chan_t: c_int, flip_s: bool, flip_t: bool, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_cut(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_invert(dst: *mut oiio_ImageBuf_t, A: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_channel_sum(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, weights: *mut oiio_CspanF32_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_contrast_remap(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, black: *mut oiio_CspanF32_t, white: *mut oiio_CspanF32_t, min: *mut oiio_CspanF32_t, max: *mut oiio_CspanF32_t, scontrast: *mut oiio_CspanF32_t, sthresh: *mut oiio_CspanF32_t, param08: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_saturate(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, scale: c_float, firstchannel: c_int, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_unsharp_mask(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, kernel: *mut oiio_StringView_t, width: c_float, contrast: c_float, threshold: c_float, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_premult(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_unpremult(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_repremult(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_convolve(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, kernel: *const oiio_ImageBuf_t, normalize: bool, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_from_kernel(name: *mut oiio_StringView_t, width: c_float, height: c_float, depth: c_float, normalize: bool, _result: *mut *mut oiio_ImageBuf_t) -> c_int;

pub fn oiio_ImageBufAlgo_computePixelHashSHA1(src: *const oiio_ImageBuf_t, extrainfo: *mut oiio_StringView_t, roi: oiio_ROI_t, blocksize: c_int, nthreads: c_int, _result: *mut *mut oiio_String_t) -> c_int;

pub fn oiio_ImageBufAlgo_crop(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, roi: oiio_ROI_t, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageBufAlgo_channels(dst: *mut oiio_ImageBuf_t, src: *const oiio_ImageBuf_t, nchannels: c_int, channelorder: *mut oiio_CspanI32_t, channelvalues: *mut oiio_CspanF32_t, newchannelnames: *mut oiio_CspanString_t, shuffle_channel_names: bool, nthreads: c_int, _result: *mut bool) -> c_int;

pub fn oiio_ImageCacheSharedPtr_ctor(ptr: *mut oiio_ImageCache_t, _result: *mut *mut oiio_ImageCacheSharedPtr_t) -> c_int;

pub fn oiio_ImageCache_attribute(_this: *mut oiio_ImageCache_t, name: *const c_char, type_: oiio_TypeDesc_t, data: *const c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageCache_getattribute(_this: *const oiio_ImageCache_t, name: *const c_char, type_: oiio_TypeDesc_t, data: *mut c_void, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_open_with_ioproxy(filename: *const c_char, spec: *const oiio_ImageSpec_t, ioproxy: *mut oiio_IOProxy_t, _result: *mut *mut oiio_ImageInputPtr_t) -> c_int;

pub fn oiio_ImageInput_open(_this: *mut oiio_ImageInput_t, filename: *const c_char, spec: *mut oiio_ImageSpec_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_open_with_config(_this: *mut oiio_ImageInput_t, filename: *const c_char, spec: *mut oiio_ImageSpec_t, config: *const oiio_ImageSpec_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageInput_create_with_ioproxy(filename: *const c_char, doopen: bool, config: *const oiio_ImageSpec_t, ioproxy: *mut oiio_IOProxy_t, plugin_searchpath: *const c_char, _result: *mut *mut oiio_ImageInputPtr_t) -> c_int;

pub fn oiio_ImageInput_supports(_this: *const oiio_ImageInput_t, feature: *const c_char, _result: *mut c_int) -> c_int;

pub fn oiio_ImageInput_valid_file(_this: *const oiio_ImageInput_t, filename: *const c_char, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_create(filename: *const c_char, ioproxy: *mut oiio_IOProxy_t, plugin_searchpath: *const c_char, _result: *mut *mut oiio_ImageOutputPtr_t) -> c_int;

pub fn oiio_ImageOutput_supports(_this: *const oiio_ImageOutput_t, feature: *const c_char, _result: *mut c_int) -> c_int;

pub fn oiio_ImageOutput_open(_this: *mut oiio_ImageOutput_t, filename: *const c_char, spec: *const oiio_ImageSpec_t, openmode: oiio_OpenMode, _result: *mut bool) -> c_int;

pub fn oiio_ImageOutput_open_multi_subimage(_this: *mut oiio_ImageOutput_t, filename: *const c_char, num_subimages: c_int, specs: *const oiio_ImageSpec_t, _result: *mut bool) -> c_int;

pub fn oiio_roi_union(A: *const oiio_ROI_t, B: *const oiio_ROI_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_roi_intersection(A: *const oiio_ROI_t, B: *const oiio_ROI_t, _result: *mut oiio_ROI_t) -> c_int;

pub fn oiio_ImageSpec_set_format_with_typename(_this: *mut oiio_ImageSpec_t, fmt: *const c_char) -> c_int;

pub fn oiio_ImageSpec_attribute(_this: *mut oiio_ImageSpec_t, name: *const c_char, type_: oiio_TypeDesc_t, data: *const c_void) -> c_int;

pub fn oiio_ImageSpec_erase_attribute(_this: *mut oiio_ImageSpec_t, name: *const c_char, search_type: oiio_TypeDesc_t, casesensitive: bool) -> c_int;

pub fn oiio_ImageSpec_find_attribute(_this: *mut oiio_ImageSpec_t, name: *const c_char, type_: oiio_TypeDesc_t, casesensitive: bool, _result: *mut *mut oiio_ParamValue_t) -> c_int;

pub fn oiio_ImageSpec_find_attribute_const(_this: *const oiio_ImageSpec_t, name: *const c_char, type_: oiio_TypeDesc_t, casesensitive: bool, _result: *mut *const oiio_ParamValue_t) -> c_int;

pub fn oiio_ImageSpec_getattributetype(_this: *const oiio_ImageSpec_t, name: *const c_char, casesensitive: bool, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_ImageSpec_getattribute(_this: *const oiio_ImageSpec_t, name: *const c_char, type_: oiio_TypeDesc_t, value: *mut c_void, casesensitive: bool, _result: *mut bool) -> c_int;

pub fn oiio_ImageSpec_decode_compression_metadata(_this: *const oiio_ImageSpec_t, name: *const c_char, defaultqual: c_int, compression: *mut *const c_char, compression_len: *mut c_longlong, compression_quality: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_channel_name(_this: *const oiio_ImageSpec_t, chan: c_int, name: *mut *const c_char, len: *mut c_longlong) -> c_int;

pub fn oiio_ImageSpec_channelindex(_this: *const oiio_ImageSpec_t, name: *const c_char, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_get_x(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_x(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_y(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_y(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_z(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_z(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_width(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_width(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_height(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_height(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_depth(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_depth(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_full_x(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_full_x(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_full_y(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_full_y(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_full_z(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_full_z(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_full_width(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_full_width(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_full_height(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_full_height(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_full_depth(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_full_depth(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_tile_width(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_tile_width(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_tile_height(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_tile_height(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_tile_depth(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_tile_depth(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_nchannels(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_nchannels(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_format_basetype(self_: *const oiio_ImageSpec_t, _result: *mut c_uchar) -> c_int;

pub fn oiio_ImageSpec_set_format_basetype(self_: *mut oiio_ImageSpec_t, value: c_uchar) -> c_int;

pub fn oiio_ImageSpec_get_channelformats_ref(self_: *const oiio_ImageSpec_t, _result: *mut *const oiio_TypeDescVector_t) -> c_int;

pub fn oiio_ImageSpec_set_channelformats(self_: *mut oiio_ImageSpec_t, value: *mut oiio_TypeDescVector_t) -> c_int;

pub fn oiio_ImageSpec_get_alpha_channel(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_alpha_channel(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_z_channel(self_: *const oiio_ImageSpec_t, _result: *mut c_int) -> c_int;

pub fn oiio_ImageSpec_set_z_channel(self_: *mut oiio_ImageSpec_t, value: c_int) -> c_int;

pub fn oiio_ImageSpec_get_deep(self_: *const oiio_ImageSpec_t, _result: *mut bool) -> c_int;

pub fn oiio_ImageSpec_set_deep(self_: *mut oiio_ImageSpec_t, value: bool) -> c_int;

pub fn oiio_ImageSpec_clear_and_reserve_channelformats(self_: *mut oiio_ImageSpec_t, size: usize) -> c_int;

pub fn oiio_ImageSpec_push_channelformat(self_: *mut oiio_ImageSpec_t, value: oiio_TypeDesc_t) -> c_int;

pub fn oiio_ImageSpec_clear_and_reserve_channelnames(self_: *mut oiio_ImageSpec_t, size: usize) -> c_int;

pub fn oiio_ImageSpec_push_channelname(self_: *mut oiio_ImageSpec_t, value: *const oiio_String_t) -> c_int;

pub fn oiio_ParamValue_ctor(name: *const oiio_ustring_t, type_: oiio_TypeDesc_t, nvalues: c_int, interp: oiio_Interp, value: *const c_void, copy: bool, _result: *mut *mut oiio_ParamValue_t) -> c_int;

pub fn oiio_ParamValueList_find(_this: *mut oiio_ParamValueList_t, name: *const c_char, type_: oiio_TypeDesc_t, casesensitive: bool, _result: *mut *mut oiio_ParamValue_t) -> c_int;

pub fn oiio_ParamValueList_find_const(_this: *const oiio_ParamValueList_t, name: *const c_char, type_: oiio_TypeDesc_t, casesensitive: bool, _result: *mut *const oiio_ParamValue_t) -> c_int;

pub fn oiio_ParamValueList_get_int(_this: *const oiio_ParamValueList_t, name: *const c_char, defaultval: c_int, casesensitive: bool, convert: bool, _result: *mut c_int) -> c_int;

pub fn oiio_ParamValueList_get_float(_this: *const oiio_ParamValueList_t, name: *const c_char, defaultval: c_float, casesensitive: bool, convert: bool, _result: *mut c_float) -> c_int;

pub fn oiio_ParamValueList_get_string(_this: *const oiio_ParamValueList_t, name: *const c_char, defaultval: *const c_char, casesensitive: bool, convert: bool, result: *mut *const c_char, len: *mut c_longlong) -> c_int;

pub fn oiio_ParamValueList_remove(_this: *mut oiio_ParamValueList_t, name: *const c_char, type_: oiio_TypeDesc_t, casesensitive: bool) -> c_int;

pub fn oiio_ParamValueList_contains(_this: *mut oiio_ParamValueList_t, name: *const c_char, type_: oiio_TypeDesc_t, casesensitive: bool, _result: *mut bool) -> c_int;

pub fn oiio_ParamValueList_attribute(_this: *mut oiio_ParamValueList_t, name: *const c_char, type_: oiio_TypeDesc_t, nvalues: c_int, data: *const c_void) -> c_int;

pub fn oiio_ParamValueList_getattributetype(_this: *const oiio_ParamValueList_t, name: *const c_char, casesensitive: bool, _result: *mut oiio_TypeDesc_t) -> c_int;

pub fn oiio_ParamValueList_getattribute(_this: *const oiio_ParamValueList_t, name: *const c_char, type_: oiio_TypeDesc_t, value: *mut c_void, casesensitive: bool, _result: *mut bool) -> c_int;

pub fn oiio_TextureSystem_texture_handle(self_: *mut oiio_TextureSystemSharedPtr_t, file_name: *mut oiio_ustring_t, per_thread: *mut oiio_Perthread_t, _result: *mut *mut oiio_TextureHandle_t) -> c_int;

pub fn oiio_TextureSystem_texture(self_: *mut oiio_TextureSystemSharedPtr_t, texture_handle: *mut oiio_TextureHandle_t, per_thread: *mut oiio_Perthread_t, options: *mut oiio_TextureOpt_v2_t, s: c_float, t: c_float, ds_dx: c_float, dt_dx: c_float, ds_dy: c_float, dt_dy: c_float, channel_count: c_int, result: *mut c_float, d_result_ds: *mut c_float, d_result_dt: *mut c_float) -> c_int;

pub fn oiio_TextureSystem_make_texture_options(first_channel: c_int, sub_image: c_int, sub_image_name: *const c_char, s_wrap: oiio_Wrap, t_wrap: oiio_Wrap, mip_mode: oiio_MipMode, interpolation_mode: oiio_InterpMode, anisotropic_samples: c_int, conservative_filter: bool, s_blur: c_float, t_blur: c_float, s_width: c_float, t_width: c_float, fill: c_float, missing_color: *mut c_float, random: c_float, r_wrap: oiio_Wrap, r_blur: c_float, r_width: c_float, dest: *mut oiio_TextureOpt_v2_t) -> c_int;

pub fn oiio_TypeDesc_fromstring(_this: *mut oiio_TypeDesc_t, typestring: *const c_char, _result: *mut usize) -> c_int;

}

