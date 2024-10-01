use crate::*;
use core::ffi::{c_char, c_size_t};
use std::mem::MaybeUninit;
use ustr::Ustr;

/// A fast alternative to [`str`](std::str) or [`String`](std::string::String)
/// for string constants.
///
/// This wraps the C++ [`ustring`](https://openimageio.readthedocs.io/en/latest/imageioapi.html#_CPPv4N4OIIO7ustringE)
/// type.
///
/// It enables many speed advantages for assignment and equality/inequality
/// testing.
///
/// > *There is a Rust version of this in the [`ustr`](https://crates.io/crates/ustr)
/// > crate, but it is [not yet binary-compatible](https://github.com/anderslanglands/ustr/issues/48)
/// > with the C++ version wrapped by this struct.*
/// >
/// > *A [`From<Ustr>`](#impl-From<Ustr>-for-Ustring) trait is provided for
/// > conversion.*
///
/// Behind the scene the implementation keeps a hash set of allocated strings,
/// so the characters of each string are unique. A `Ustring` itself is a
/// pointer to the characters of one of these canonical strings. Therefore,
/// assignment and equality testing is just a single 32- or 64-bit int
/// operation, the only lock occurs is when an `Ustring` is created from raw
/// characters, and the only allocation is the first time each canonical
/// `Ustring` is created.
///
/// The internal table also contains a [`CString`](std::ffi::CString) version so
/// converting a `Ustring` to a `str` (via [`Ustring::as_str()`]) or querying
/// the number of characters (via [`Ustring::len()`]) is extremely inexpensive.
///
/// Usage guidelines:
///
/// Compared to standard strings, `Ustring`s have several advantages:
///
///   - Each individual `Ustring` is very small -- in fact, we guarantee that a
///     `Ustring` is the same size and memory layout as an ordinary
///     [`CStr`](std::ffi::CStr).
///
///   - Storage is frugal, since there is only one allocated copy of each unique
///     character sequence, throughout the lifetime of the program.
///
///   - Assignment from one `Ustring` to another is just copy of the pointer; no
///     allocation, no character copying, no reference counting.
///
///   - Equality testing (do the strings contain the same characters) is a
///     single operation, the comparison of the pointer.
///
///   - Memory allocation only occurs when a new `Ustring` is constructed from
///     raw characters the *first* time -- subsequent constructions of the same
///     string just finds it in the canonical string set, but doesn't need to
///     allocate new storage. Destruction of a `Ustring` is trivial, there is no
///     de-allocation because the canonical version stays in the set.  Also,
///     therefore, no user code mistake can lead to memory leaks.
///
/// But there are some problems, too. Canonical strings are never freed
/// from the table. So in some sense all the strings "leak", but they
/// only leak one copy for each unique string that the program ever comes
/// across.  Also, creation of unique strings from raw characters is more
/// expensive than for standard strings, due to hashing, table queries,
/// and other overhead.
///
/// On the whole, `Ustrings` are a really great string representation
///
///   - if you tend to have (relatively) few unique strings, but many copies of
///     those strings;
///
///   - if the creation of strings from raw characters is relatively rare
///     compared to copying or comparing to existing strings;
///
///   - if you tend to make the same strings over and over again, and if it's
///     relatively rare that a single unique character sequence is used only
///     once in the entire lifetime of the program;
///
///   - if your most common string operations are assignment and equality
///     testing and you want them to be as fast as possible;
///
///   - if you are doing relatively little character-by-character assembly of
///     strings, string concatenation, or other "string manipulation" (other
///     than equality testing).
///
/// `Ustring`s are not so hot
///
///   - if your program tends to have very few copies of each character sequence
///     over the entire lifetime of the program;
///
///   - if your program tends to generate a huge variety of unique strings over
///     its lifetime, each of which is used only a short time and then
///     discarded, never to be needed again;
///
///   - if you don't need to do a lot of string assignment or equality testing,
///     but lots of more complex string manipulation.
pub struct Ustring {
    ptr: *mut oiio_ustring_t,
}

impl From<Ustr> for Ustring {
    fn from(s: Ustr) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ustring_t>::uninit();

        unsafe {
            oiio_ustring_new(s.as_char_ptr(), &mut ptr as *mut _ as _);

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }
}

impl From<&str> for Ustring {
    fn from(s: &str) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ustring_t>::uninit();

        unsafe {
            oiio_ustring_new_from_parts(s.as_ptr() as _, s.len() as _, &mut ptr as *mut _ as _);

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }
}

impl Ustring {
    pub fn len(&self) -> usize {
        let mut result = MaybeUninit::<c_size_t>::uninit();

        unsafe {
            oiio_ustring_size(self.ptr, &mut result as *mut _ as _);
            result.assume_init() as _
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns a static string slice. It is `'static` as all `Ustring`s
    /// lifetime
    pub fn as_str(&self) -> &'static str {
        let mut ptr = MaybeUninit::<*const c_char>::uninit();
        let mut len = MaybeUninit::<c_size_t>::uninit();

        unsafe {
            oiio_ustring_c_str(self.ptr, &mut ptr as *mut _ as _);
            oiio_ustring_size(self.ptr, &mut len as *mut _ as _);

            std::str::from_raw_parts(ptr.assume_init() as _, len.assume_init() as _)
        }
    }
}

#[cfg(feature = "ffi")]
impl Ustring {
    /// Returns a pointer to the underlying [`oiio_ustring_t`].
    pub fn as_raw_ptr(&self) -> *const oiio_ustring_t {
        self.ptr
    }

    /// Returns a mutable pointer to the underlying [`oiio_ustring_t`].
    pub fn as_raw_ptr_mut(&mut self) -> *mut oiio_ustring_t {
        self.ptr
    }
}

impl Drop for Ustring {
    fn drop(&mut self) {
        unsafe {
            oiio_ustring_dtor(self.ptr);
        }
    }
}
