extern crate orbclient;

use orbclient::{Window, Renderer, get_display_size};
use std::os::raw::*;
use std::mem::transmute;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn orb_display_width() -> u32 {
    get_display_size().unwrap().0
}

#[no_mangle]
pub unsafe extern "C" fn orb_display_height() -> u32 {
    get_display_size().unwrap().1
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_new(
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    title: *const c_char,
) ->  *mut Window {
    let title = CStr::from_ptr(title).to_string_lossy();
    transmute(Box::new(Window::new(x, y, w, h, &title).unwrap()))
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_data(window: &mut Window) ->  *mut u32 {
    window.data_mut().as_mut_ptr() as *mut u32
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_width(window: &mut Window) -> u32 {
    window.width()
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_height(window: &mut Window) -> u32 {
    window.height()
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_x(window: &mut Window) -> i32 {
    window.x()
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_y(window: &mut Window) -> i32 {
    window.y()
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_set_pos(window: &mut Window, x: i32, y: i32) {
    window.set_pos(x, y);
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_set_size(window: &mut Window, w: u32, h: u32) {
    window.set_size(w, h);
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_set_title(window: &mut Window, title: *const c_char) {
    let title = CStr::from_ptr(title).to_string_lossy();
    window.set_title(&title);
}

#[no_mangle]
pub unsafe extern "C" fn orb_window_sync(window: &mut Window) {
    window.sync();
}
