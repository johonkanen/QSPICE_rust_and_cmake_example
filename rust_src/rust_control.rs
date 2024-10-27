// Externally accessible data structure
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

// DLL main function for initialization
#[no_mangle]
pub extern "stdcall" fn dll_main(_module: *mut c_void, _reason: u32, _reserved: *mut c_void) -> i32 {
    1 // Success
}

// module start here

extern crate rand;
use rand::Rng;
use std::ffi::c_void;
use std::sync::Mutex;
use once_cell::sync::Lazy;

mod pi_control;
pub use pi_control::PIController;

// Constants and static variables
static CALCULATION_STEP: f64 = 10.0e-6;
static REFERENCE: f64 = -2.5;
static KP: f64 = 0.5;
static KI: f64 = 0.0625;

static mut T_NEXT: f64 = 0.0;

// Initialize PIController globally using Lazy and Mutex for thread safety
static CONTROLLER: Lazy<Mutex<PIController>> = Lazy::new(|| {
    Mutex::new(PIController::new(KP, KI))
});


#[no_mangle]
pub extern "C" fn control(_opaque: *mut *mut c_void, t: f64, data: *mut uData) {
    unsafe {
        // Retrieve input and output pointers
        let uout        = (*data.add(0)).d;
        let control_ref = &mut (*data.add(1)).d;
        let load_ref    = &mut (*data.add(2)).d;

        // Control logic with timing check
        if t >= T_NEXT {
            T_NEXT = t + CALCULATION_STEP;
            
            // Lock the global controller and compute control output
            let mut controller = CONTROLLER.lock().unwrap();
            *control_ref = controller.compute_control(REFERENCE, uout);
        }

        // Load reference with random disturbance
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

