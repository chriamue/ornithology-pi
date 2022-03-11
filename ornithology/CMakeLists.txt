cmake_minimum_required(VERSION 3.16)

project(ornithology VERSION 0.7.0 LANGUAGES CXX)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.2 COMPONENTS Bluetooth Quick QuickControls2 Gui REQUIRED)

qt_add_executable(app${PROJECT_NAME}
    MANUAL_FINALIZATION
    main.cpp
    client.h client.cpp
    device.h device.cpp
    deviceinfo.h deviceinfo.cpp
    message.h message.cpp
    sighting.h sighting.cpp
    resources.qrc
    android/AndroidManifest.xml
)

qt_add_qml_module(app${PROJECT_NAME}
    URI ornithology
    VERSION 0.7.0
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

set_target_properties(app${PROJECT_NAME} PROPERTIES
    QT_ANDROID_PACKAGE_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/android"
    )

set_target_properties(app${PROJECT_NAME} PROPERTIES
    QT_ANDROID_VERSION_NAME "0.7"
    )

set_target_properties(app${PROJECT_NAME} PROPERTIES
    QT_ANDROID_VERSION_CODE "2"
    )

set_target_properties(app${PROJECT_NAME} PROPERTIES
    MACOSX_BUNDLE_GUI_IDENTIFIER ornithology.chriamue.net
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

target_compile_definitions(app${PROJECT_NAME}
    PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)
target_link_libraries(app${PROJECT_NAME}
    PRIVATE Qt6::Bluetooth Qt6::Quick Qt6::QuickControls2 Qt6::Gui)

qt_finalize_executable(app${PROJECT_NAME})
