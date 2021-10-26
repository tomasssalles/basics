cmake_minimum_required(VERSION 3.10)

project(basics VERSION 1.0)

set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)

configure_file(basics_config.h.in basics_config.h)

add_subdirectory(sorting)
add_subdirectory(tests)

add_executable(play playground.c)

target_link_libraries(play PUBLIC
	sorting
)

target_include_directories(play PUBLIC
	"${PROJECT_BINARY_DIR}"
	"${PROJECT_SOURCE_DIR}/sorting"
)

enable_testing()
add_test(NAME bubble_sort COMMAND test_bubble_sort)
