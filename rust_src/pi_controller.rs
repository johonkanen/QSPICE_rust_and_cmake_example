// #[link(name = "kernel32")]
extern "system" {}

use std::ffi::c_void;
use rand::Rng;

// Declare the module with module source file name
mod pi_control;
pub use pi_control::PIController;

#[repr(C)]
pub union uData {
    pub b: bool,
    pub c: i8,
    pub uc: u8,
    pub s: i16,
    pub us: u16,
    pub i: i32,
    pub ui: u32,
    pub f: f32,
    pub d: f64,
    pub i64: i64,
    pub ui64: u64,
    pub str: *mut i8,
    pub bytes: *mut u8,
}

// Initialization atomic flag
pub extern "stdcall" fn dll_main(_module: *mut c_void, _reason: u32, _reserved: *mut c_void) -> i32 {
    1 // Success
}

// Constants and mutable static variables
static CALCULATION_STEP: f64 = 10.0e-6;
static REFERENCE: f64 = -2.5;
static KP: f64 = 0.5;
static KI: f64 = 0.0625;

static mut T_NEXT: f64 = 0.0;
static mut INTEGRAL: f64 = 0.0;

#[no_mangle]
pub extern "C" fn control(_opaque: *mut *mut c_void, t: f64, data: *mut uData) {
    unsafe {
        let uout        = (*data.add(0)).d;
        let control_ref = &mut (*data.add(1)).d;
        let load_ref    = &mut (*data.add(2)).d;

        if t >= T_NEXT {
            T_NEXT = t + CALCULATION_STEP;
            let error = REFERENCE - uout;

            *control_ref = KP * error + INTEGRAL;
            INTEGRAL     = KI * error + INTEGRAL;
        }

        let random_value: f64 = rand::thread_rng().gen_range(-1.0..=1.0);

        *load_ref = if t > 20e-3 {
            random_value - 10.0
        } else if t > 10e-3 {
            random_value + 10.0
        } else {
            random_value
        };
    }
}
