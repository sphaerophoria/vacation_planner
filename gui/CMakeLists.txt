project(gui CXX)

add_compile_options(-Wall -Wextra -Wpedantic)
find_package(Qt5 COMPONENTS Quick REQUIRED)
add_library(vacation_planner_gui SHARED gui.cpp res/res.qrc)
target_link_libraries(vacation_planner_gui PUBLIC Qt5::Quick)
set_target_properties(vacation_planner_gui PROPERTIES AUTOMOC true AUTORCC true)
target_compile_features(vacation_planner_gui PRIVATE cxx_std_17)

install(TARGETS vacation_planner_gui)
