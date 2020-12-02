#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub a: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo__bindgen_ty_1 {
    pub b: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: foo__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_1 {
    pub c1: ::std::os::raw::c_ushort,
    pub c2: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1__bindgen_ty_1>())).c1
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(c1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1__bindgen_ty_1>())).c2
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(c2)
        )
    );
}
struct Box_foo__bindgen_ty_1__bindgen_ty_1 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_foo__bindgen_ty_1__bindgen_ty_1 {}
impl Drop for Box_foo__bindgen_ty_1__bindgen_ty_1 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 2usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_2 {
    pub d1: ::std::os::raw::c_uchar,
    pub d2: ::std::os::raw::c_uchar,
    pub d3: ::std::os::raw::c_uchar,
    pub d4: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1__bindgen_ty_2>())).d1
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(d1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1__bindgen_ty_2>())).d2
                as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(d2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1__bindgen_ty_2>())).d3
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(d3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1__bindgen_ty_2>())).d4
                as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(d4)
        )
    );
}
struct Box_foo__bindgen_ty_1__bindgen_ty_2 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_foo__bindgen_ty_1__bindgen_ty_2 {}
impl Drop for Box_foo__bindgen_ty_1__bindgen_ty_2 {
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
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
}
impl Default for foo__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        8usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
}
impl Default for foo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_foo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_foo {}
impl Drop for Box_foo {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 4usize).unwrap(),
            );
        }
    }
}
