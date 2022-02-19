import QtQuick 2

Window {
    id: mainWindow
    width: 640
    height: 480
    visible: true
    title: qsTr("Ornithology App")

    Image {
        id: bluetoothImage
        x: 72
        y: 50
        width: 100
        height: 100
        source: "resources/bluetooth.png"
        fillMode: Image.PreserveAspectFit

        MouseArea {
            anchors.fill: parent
            onClicked: {
                bluetooth.startDeviceDiscovery()
            }
        }
    }

    ListView {
        id: deviceListView
        width: 200; height: 300
        spacing: 5
        model: deviceList
        delegate: Rectangle {
            required property string name
            height: 25
            width: 200
            color: "lightgray"
            Text { text: parent.name; anchors.centerIn: parent }
        }
    }
}
