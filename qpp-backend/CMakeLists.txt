cmake_minimum_required(VERSION 3.15)
project(qpp-agent)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_PREFIX_PATH "/home/lucky/Code/qpp/install")

# If Quantum++'s installation path was non-standard, i.e., specified by
#
# cmake -B build -DCMAKE_INSTALL_PREFIX=/path/to/installed/qpp
#
# then uncomment the following line and replace the installation path with yours

# set(CMAKE_PREFIX_PATH "/path/to/installed/qpp")

find_package(qpp REQUIRED)
include_directories(include)
add_executable(qpp-agent src/main.cpp)
target_link_libraries(qpp-agent PUBLIC ${QPP_LINK_DEPS} libqpp)
