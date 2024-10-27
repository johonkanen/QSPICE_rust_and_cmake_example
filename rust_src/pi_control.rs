// rust_src/pi_controller.rs

pub struct PIController {
    pub kp: f64,        // Proportional gain
    pub ki: f64,        // Integral gain
    integral: f64,      // Integral term
}

impl PIController {
    pub fn new(kp: f64, ki: f64) -> Self {
        PIController {
            kp,
            ki,
            integral: 0.0,
        }
    }

    pub fn compute_control(&mut self, reference: f64, measurement: f64) -> f64 {
        let error = reference - measurement;
        let control_out = self.kp * error + self.integral;
        self.integral += self.ki * error;
        control_out
    }
}
