cmake_minimum_required(VERSION 3.20)

project(
    nexus_blink
    VERSION 0.1.0
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(BLINK_SOURCE_DIR
    ${CMAKE_CURRENT_LIST_DIR}/../src
)

set(BLINK_INCLUDE_DIR
    ${CMAKE_CURRENT_LIST_DIR}/../include
)

set(BLINK_SOURCES
    ${BLINK_SOURCE_DIR}/blink_engine.cpp
    ${BLINK_SOURCE_DIR}/rendering_context.cpp
    ${BLINK_SOURCE_DIR}/document.cpp
    ${BLINK_SOURCE_DIR}/frame.cpp
    ${BLINK_SOURCE_DIR}/page.cpp
)

add_library(
    nexus_blink
    SHARED
    ${BLINK_SOURCES}
)

target_include_directories(
    nexus_blink
    PUBLIC
        ${BLINK_INCLUDE_DIR}
    PRIVATE
        ${BLINK_SOURCE_DIR}
)

target_compile_features(
    nexus_blink
    PUBLIC
        cxx_std_20
)

if(MSVC)
    target_compile_options(
        nexus_blink
        PRIVATE
            /W4
            /permissive-
        )
else()
    target_compile_options(
        nexus_blink
        PRIVATE
            -Wall
            -Wextra
            -Wpedantic
            -Wconversion
            -Wshadow
        )
endif()

set_target_properties(
    nexus_blink
    PROPERTIES
        OUTPUT_NAME "nexus_blink"
        VERSION ${PROJECT_VERSION}
        SOVERSION 0
)

install(
    TARGETS nexus_blink
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
    ARCHIVE DESTINATION lib
)

install(
    DIRECTORY ${BLINK_INCLUDE_DIR}/
    DESTINATION include/nexus/blink
    FILES_MATCHING
    PATTERN "*.hpp"
)
