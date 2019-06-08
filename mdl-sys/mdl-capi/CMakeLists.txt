cmake_minimum_required(VERSION 3.5)
project(mdl-capi)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(mdl-capi STATIC 
    mdl-capi.cpp
    neuray-capi.cpp
    imdl_compiler-capi.cpp
    idatabase-capi.cpp
    imdl_factory-capi.cpp
    imdl_execution_context-capi.cpp
    iimage_api-capi.cpp
    iscope-capi.cpp
    iversion-capi.cpp
    itransaction-capi.cpp
    ifunction_definition-capi.cpp
    imaterial_definition-capi.cpp
    imodule-capi.cpp
    iinterface-capi.cpp
    itype_list-capi.cpp
    itype-capi.cpp
    ivalue-capi.cpp
    iexpression-capi.cpp
    )
target_include_directories(mdl-capi PUBLIC ${INC_MDL})
# target_link_libraries(mdl-capi PUBLIC)
target_compile_options(mdl-capi PRIVATE "-Wno-deprecated-register")

install(TARGETS mdl-capi DESTINATION ${CMAKE_INSTALL_PREFIX})

