cmake_minimum_required(VERSION 3.10)
project(My32BitDLL)

# Function to create a shared library
function(create_shared_library lib_name source_files)

    add_library(${lib_name} SHARED ${source_files})

    target_compile_definitions(${lib_name} PRIVATE MYLIB_EXPORTS)
    set_target_properties(${lib_name} PROPERTIES PREFIX "") #remove lib prefix
    
    target_link_libraries(${lib_name} PRIVATE kernel32)
    
endfunction()

# Add the source files
set(cpp_control_source_files
    cpp_src/pi_control.hpp
    cpp_src/pi_control.cpp
    cpp_src/cmake_control.cpp
)

# Ensure the target is built as a 32-bit DLL
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -m32")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -m32")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -m32")

# Link with kernel32.dll
create_shared_library(control "${cpp_control_source_files}")
