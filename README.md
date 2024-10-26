# QSPICE_rust_and_cmake_example
Test building rust and C++ via cmake for qspice

to build the source directly from QSPICE, open qspice/controlled_current.qsh

To build with cmake, 
> mkdir build
 cd build
 cmake -G "Ninja" -DCMAKE_C_COMPILER="d:\msys64\mingw32\bin\clang.exe" -DCMAKE_CXX_COMPILER="d:\msys64\mingw32\bin\clang++.exe" .. 
 cmake --build .

change the paths to clang and clang++ to your paths where clang compilers are found


then copy the created .dll to the same folder with controlled_current.qsch


to build the control.dll with rust, run

> cargo build --release --target=i686-pc-windows-msvc

the tools can be installed with rustup package manager using
> rustup target add i686-pc-windows-msvc

cmake and 32 bit clang compilers can be installed with msys2 using
> pacman -S mingw-w64-i686-clang

and 

> pacman -S mingw-w64-x86_64-cmake

note, that installing rust from msys2 does not install rustup, so better

https://www.rust-lang.org/tools/install
