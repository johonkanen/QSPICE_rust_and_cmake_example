# QSPICE_rust_and_cmake_example

Test building Rust and C++ via CMake for QSPICE.

## Schematic and Simulation Response

Below are the schematic and simulation response figures for this project:

### Schematic
![Schematic](figs/schematic.png)

### Simulation Response
![Simulation Response](figs/simulation_response.png)

## Building the Project

To build the source directly from QSPICE, open `qspice/controlled_current.qsh`.

To build with CMake:

```bash
> mkdir build
cd build
cmake -G "Ninja" -DCMAKE_C_COMPILER="d:\msys64\mingw32\bin\clang.exe" -DCMAKE_CXX_COMPILER="d:\msys64\mingw32\bin\clang++.exe" ..
cmake --build .

