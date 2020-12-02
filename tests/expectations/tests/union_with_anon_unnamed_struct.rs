#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union pixel {
    pub rgba: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: pixel__bindgen_ty_1,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct pixel__bindgen_ty_1 {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
    pub a: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_pixel__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<pixel__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(pixel__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<pixel__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(pixel__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<pixel__bindgen_ty_1>())).r as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel__bindgen_ty_1),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<pixel__bindgen_ty_1>())).g as *const _
                as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel__bindgen_ty_1),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<pixel__bindgen_ty_1>())).b as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<pixel__bindgen_ty_1>())).a as *const _
                as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel__bindgen_ty_1),
            "::",
            stringify!(a)
        )
    );
}
struct Box_pixel__bindgen_ty_1 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_pixel__bindgen_ty_1 {}
impl Drop for Box_pixel__bindgen_ty_1 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 1usize).unwrap(),
            );
        }
    }
}
#[test]
fn bindgen_test_layout_pixel() {
    assert_eq!(
        ::std::mem::size_of::<pixel>(),
        4usize,
        concat!("Size of: ", stringify!(pixel))
    );
    assert_eq!(
        ::std::mem::align_of::<pixel>(),
        4usize,
        concat!("Alignment of ", stringify!(pixel))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pixel>())).rgba as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pixel),
            "::",
            stringify!(rgba)
        )
    );
}
impl Default for pixel {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
