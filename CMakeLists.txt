cmake_minimum_required(VERSION 3.10)
project(My32BitDLL)

# Function to create a shared library and copy .dll to destination folder
function(create_shared_library lib_name source_files destination_folder)

    add_library(${lib_name} SHARED ${source_files})

    target_compile_definitions(${lib_name} PRIVATE MYLIB_EXPORTS)
    set_target_properties(${lib_name} PROPERTIES PREFIX "") #remove lib prefix
    
    target_link_libraries(${lib_name} PRIVATE kernel32)
    
    # Custom command to copy the DLL after build
    add_custom_command(TARGET ${lib_name} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
            $<TARGET_FILE:${lib_name}>
            ${destination_folder}/${lib_name}.dll
    )

endfunction()


# Add the source files
set(peak_shave_48v_sources
    spice_circuits/qspice/control.cpp
)

# Ensure the target is built as a 32-bit DLL
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -m32")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -m32")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -m32")

# Link with kernel32.dll
create_shared_library(control "${peak_shave_48v_sources}" ${CMAKE_SOURCE_DIR}/spice_circuits/qspice)

