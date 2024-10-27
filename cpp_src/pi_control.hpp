// PIController.h

class PIController {
public:
    // Constructor to initialize kp and ki with provided values
    PIController(double kp, double ki);

    // Method to compute the control output based on reference and measurement
    double compute_control(double reference, double measurement);

private:
    double kp;       // Proportional gain
    double ki;       // Integral gain
    double integral; // Integral term
};

