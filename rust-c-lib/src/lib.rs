use libc::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct EasyStruct {
    pub value: u8,
}

#[repr(C)]
pub struct ComplexStruct {
    name: *const c_char,
    values: *const u8,
    values_len: usize,
}

#[no_mangle]
pub extern "C" fn get_easy_struct(value: u8) -> *const EasyStruct {
    Box::into_raw(Box::new(EasyStruct { value }))
}

/// # Safety
/// Call this method when you are done working with an instance of EasyStruct.
/// Will not do anything, when a NULL-Pointer is passed as argument.
#[no_mangle]
pub unsafe extern "C" fn free_easy_struct(easy_struct: *const EasyStruct) {
    unsafe {
        if !easy_struct.is_null() {
            let _ = Box::from_raw(easy_struct as *mut EasyStruct);
        }
    }
}

#[no_mangle]
pub extern "C" fn get_complex_struct(
    name: *const c_char,
    values: *const u8,
    values_len: usize,
) -> *const ComplexStruct {
    if !values.is_null() {
        Box::into_raw(Box::new(ComplexStruct {
            name,
            values,
            values_len,
        }))
    } else {
        Box::into_raw(Box::new(ComplexStruct {
            name: CString::default().into_raw(),
            values: Vec::new().as_mut_ptr(),
            values_len: 0,
        }))
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_complex(complex: *const ComplexStruct) {
    unsafe {
        if !complex.is_null() {
            let _ = Box::from_raw(complex as *mut ComplexStruct);
        }
    }
}
