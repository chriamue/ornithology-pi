cmake_minimum_required(VERSION 3.16)

project(ornithology VERSION 0.1 LANGUAGES CXX)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.2 COMPONENTS Bluetooth Quick QuickControls2 Gui REQUIRED)

qt_add_executable(appornithology
    main.cpp
    client.h client.cpp
    device.h device.cpp
    deviceinfo.h deviceinfo.cpp
    message.h message.cpp
    sighting.h sighting.cpp
    resources.qrc
)

qt_add_qml_module(appornithology
    URI ornithology
    VERSION 1.0
    QML_FILES
        main.qml
        Client.qml
        CustomLabel.qml
        Devices.qml
        InfoDialog.qml
        Header.qml
        SearchMenu.qml
        ZoomImage.qml
    RESOURCES
       resources/bluetooth.png
       resources/busy_dark.png
)

set_target_properties(appornithology PROPERTIES
    MACOSX_BUNDLE_GUI_IDENTIFIER ornithology.chriamue.net
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

target_compile_definitions(appornithology
    PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)
target_link_libraries(appornithology
    PRIVATE Qt6::Bluetooth Qt6::Quick Qt6::QuickControls2 Qt6::Gui)
