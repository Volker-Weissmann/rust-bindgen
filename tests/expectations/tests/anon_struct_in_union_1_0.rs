#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct s {
    pub u: s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct s__bindgen_ty_1 {
    pub field: __BindgenUnionField<s__bindgen_ty_1_inner>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct s__bindgen_ty_1_inner {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_s__bindgen_ty_1_inner() {
    assert_eq!(
        ::std::mem::size_of::<s__bindgen_ty_1_inner>(),
        4usize,
        concat!("Size of: ", stringify!(s__bindgen_ty_1_inner))
    );
    assert_eq!(
        ::std::mem::align_of::<s__bindgen_ty_1_inner>(),
        4usize,
        concat!("Alignment of ", stringify!(s__bindgen_ty_1_inner))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<s__bindgen_ty_1_inner>())).b as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(s__bindgen_ty_1_inner),
            "::",
            stringify!(b)
        )
    );
}
impl Clone for s__bindgen_ty_1_inner {
    fn clone(&self) -> Self {
        *self
    }
}
struct Box_s__bindgen_ty_1_inner {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_s__bindgen_ty_1_inner {}
impl Drop for Box_s__bindgen_ty_1_inner {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[test]
fn bindgen_test_layout_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<s__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<s__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<s__bindgen_ty_1>())).field as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(s__bindgen_ty_1),
            "::",
            stringify!(field)
        )
    );
}
impl Clone for s__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_s() {
    assert_eq!(
        ::std::mem::size_of::<s>(),
        4usize,
        concat!("Size of: ", stringify!(s))
    );
    assert_eq!(
        ::std::mem::align_of::<s>(),
        4usize,
        concat!("Alignment of ", stringify!(s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<s>())).u as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(s), "::", stringify!(u))
    );
}
impl Clone for s {
    fn clone(&self) -> Self {
        *self
    }
}
struct Box_s {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_s {}
impl Drop for Box_s {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
