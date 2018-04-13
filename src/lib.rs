extern crate orbclient;

use orbclient::{Window, Renderer};
use std::os::raw::*;
use std::mem::transmute;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn orb_window_new(
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    title: *const c_char,
) ->  *mut c_void {
    let title = CStr::from_ptr(title).to_string_lossy();
    let window: *mut Window = transmute(Box::new(orbclient::Window::new(x, y, w, h, &title).unwrap()));
    window as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_data(window: *mut c_void) ->  *mut u32 {
    let window: &mut Window = &mut *(window as *mut Window);
    window.data_mut().as_mut_ptr() as *mut u32
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_width(window: *mut c_void) -> u32 {
    let window: &mut Window = &mut *(window as *mut Window);
    window.width()
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_height(window: *mut c_void) -> u32 {
    let window: &mut Window = &mut *(window as *mut Window);
    window.height()
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_sync(window: *mut c_void) {
    let window: &mut Window = &mut *(window as *mut Window);
    window.sync();
}
