cmake_minimum_required(VERSION 3.16)

set(This Example)

project(${This} C CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Boost REQUIRED)

enable_testing()

add_subdirectory(googletest)

set(Sources 
    src/main.cpp
    src/DeclarationFormCalculator.cpp
    src/Group.cpp
    src/SeatCalculator.cpp
    src/SolutionHandler.cpp
    src/SpecialOperator.cpp
    src/PassportValidator.cpp
)


add_library(${This} STATIC ${Sources} ${Headers})

add_executable(ExampleExec ${Sources} ${Headers})

add_subdirectory(test)

target_link_libraries(ExampleExec 
                      LINK_PUBLIC ${Boost_LIBRARIES})