// Automatically generated Rust file on Sat Oct 26 07:34:26 2024

use std::ffi::c_void;
use std::os::raw::{c_char, c_float, c_int, c_uchar, c_ulonglong, c_ushort};
use std::sync::atomic::{AtomicBool, Ordering};
use winapi::um::libloaderapi::GetModuleHandleA; // For example, from kernel32.dll

#[repr(C)]
pub union UData {
    b: bool,
    c: c_char,
    uc: c_uchar,
    s: c_ushort,
    us: c_ushort,
    i: c_int,
    ui: u32,
    f: c_float,
    d: f64,
    i64: i64,
    ui64: c_ulonglong,
    str_ptr: *mut c_char,
    bytes_ptr: *mut c_uchar,
}

// Initialization atomic flag
static INIT: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "stdcall" fn dll_main(_: *mut c_void, _: u32, _: *mut c_void) -> c_int {
    INIT.store(true, Ordering::SeqCst);

    // Example of using `GetModuleHandleA` from `kernel32.dll`
    let module_handle = unsafe { GetModuleHandleA(std::ptr::null()) };

    if module_handle.is_null() {
        eprintln!("Failed to get module handle.");
        return 0;
    }

    1
}

// Constants and mutable static variables
static CALCULATION_STEP: f64 = 10.0e-6;
static KP: f64 = 0.5;
static KI: f64 = 0.0625;

static mut t_next: f64 = 0.0;
static mut integral: f64 = 0.0;

#[no_mangle]
pub extern "C" fn control(opaque: *mut *mut c_void, t: f64, data: *mut UData) {
    unsafe {
        let uout        = (*data.add(0)).d;
        let control_ref = &mut (*data.add(1)).d;
        let load_ref    = &mut (*data.add(2)).d;

        if t >= T_NEXT {
            T_NEXT = t + CALCULATION_STEP;
            let error = 1.0 - uout;

            *control_ref = KP * error + INTEGRAL;
            INTEGRAL     = KI * error + INTEGRAL;
        }

        let random_value: f64 = rand::random::<f64>() * 2.0 - 1.0;

        *load_ref = if t > 20e-3 {
            random_value - 10.0
        } else if t > 10e-3 {
            random_value + 10.0
        } else {
            random_value
        };
    }
}
