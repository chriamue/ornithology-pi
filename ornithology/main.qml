import QtQuick 2

Window {
    id: mainWindow
    width: 640
    height: 480
    visible: true
    title: qsTr("Ornithology App")

    Loader {
        id: pageLoader
        anchors.fill: parent
        source: "Devices.qml"
    }
}
