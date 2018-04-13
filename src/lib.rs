extern crate orbclient;

use orbclient::{Window, WindowFlag, Renderer, EventIter, get_display_size};
use std::os::raw::*;
use std::mem::transmute;
use std::ffi::CStr;

use orbclient::event::*;

/// Should be in sync with `orbclient::event::EventOption`
#[repr(C)]
pub enum OrbEventOption {
    Key {
        character: char,
        scancode: u8,
        pressed: bool,
    },
    Mouse {
        x: i32,
        y: i32,
    },
    Button {
        left: bool,
        middle: bool,
        right: bool,
    },
    Scroll {
        x: i32,
        y: i32,
    },
    Quit {
    },
    Focus {
        focused: bool,
    },
    Move {
        x: i32,
        y: i32,
    },
    Resize {
        width: u32,
        height: u32,
    },
    Screen {
        width: u32,
        height: u32,
    },
    Unknown {
        code: i64,
        a: i64,
        b: i64,
    },
    None,
}

fn event_option_to_c(event_option: &EventOption) -> OrbEventOption {
    match *event_option {
        EventOption::Key(KeyEvent { character, scancode, pressed }) => OrbEventOption::Key { character, scancode, pressed },
        EventOption::Mouse(MouseEvent { x, y }) => OrbEventOption::Mouse { x, y },
        EventOption::Button(ButtonEvent { left, middle, right }) => OrbEventOption::Button { left, middle, right },
        EventOption::Scroll(ScrollEvent { x, y }) => OrbEventOption::Scroll { x, y },
        EventOption::Quit(QuitEvent { }) => OrbEventOption::Quit { },
        EventOption::Focus(FocusEvent { focused }) => OrbEventOption::Focus { focused },
        EventOption::Move(MoveEvent { x, y }) => OrbEventOption::Move { x, y },
        EventOption::Resize(ResizeEvent { width, height }) => OrbEventOption::Resize { width, height },
        EventOption::Screen(ScreenEvent { width, height }) => OrbEventOption::Screen { width, height },
        EventOption::Unknown(Event { code, a, b }) => OrbEventOption::Unknown { code, a, b },
        EventOption::None => OrbEventOption::None,
    }
}

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
) -> *mut Window {
    let title = CStr::from_ptr(title).to_string_lossy();
    transmute(Box::new(Window::new_flags(x, y, w, h, &title, &[WindowFlag::Async, WindowFlag::Resizable]).unwrap()))
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

#[no_mangle]
pub unsafe extern "C" fn orb_window_events(window: &mut Window) -> *mut EventIter {
    transmute(Box::new(window.events()))
}

#[no_mangle]
pub unsafe extern "C" fn orb_events_next(event_iter: &mut EventIter) -> OrbEventOption {
    if let Some(event) = event_iter.next() {
        event_option_to_c(&event.to_option())
    } else {
        OrbEventOption::None
    }
}
