// PIController.cpp

#include "pi_control.hpp"

PIController::PIController(double kp, double ki) : kp(kp), ki(ki), integral(0.0) {}

double PIController::compute_control(double reference, double measurement) {
    double error = reference - measurement;

    double control_out = kp * error + integral;
    integral           = ki * error + integral;

    return control_out;
}
