// Automatically generated C++ file on Sat Oct 26 07:34:26 2024
//
// To build with Digital Mars C++ Compiler:
//
//    dmc -mn -WD control.cpp kernel32.lib

#include <cstdlib> // for rand

union uData
{
   bool b;
   char c;
   unsigned char uc;
   short s;
   unsigned short us;
   int i;
   unsigned int ui;
   float f;
   double d;
   long long int i64;
   unsigned long long int ui64;
   char *str;
   unsigned char *bytes;
};

// int DllMain() must exist and return 1 for a process to load the .DLL
// See https://docs.microsoft.com/en-us/windows/win32/dlls/dllmain for more information.
int __stdcall DllMain(void *module, unsigned int reason, void *reserved) { return 1; }

// #undef pin names lest they collide with names in any header file(s) you might include.
#undef uout
#undef control
#undef load


/******************/

double t_next = 0.0;
const double calculation_step = 10.0e-6;

double integral = 0.0;

const double kp = 0.5;
const double ki = 0.0625;


extern "C" __declspec(dllexport) void control(void **opaque, double t, union uData *data)
{
   double  uout    = data[0].d; // input
   double &control = data[1].d; // output
   double &load    = data[2].d; // output

// Implement module evaluation code here:

    if (t >= t_next)
    {
        t_next = t + calculation_step;

        double error = 1.0 - uout;

        control  = kp * error + integral;
        integral = ki * error + integral;

    }
    
    double random_value = ((double)std::rand()/(double)RAND_MAX*2.0-1.0);

    load = random_value;
    if (t > 10e-3)
        load = random_value + 10.0;
    if (t > 20e-3)
        load = random_value - 10.0;

}
