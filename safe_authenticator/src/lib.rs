extern crate safe_core;

#[no_mangle]
#[doc(hidded)]
pub extern "C" fn ffi() {
    safe_core::foo();
}
