import QtQuick 2

Window {
    id: mainWindow
    width: 640
    height: 480
    visible: true
    title: qsTr("Ornithology App")

    Menu {
        id: menu
        anchors.top: parent.top
        menuWidth: parent.width
        menuHeight: (parent.height/6)
        menuText: device.update
        onButtonClick: {
            if (device.state) {
                device.deviceScanFinished();
            } else {
                device.startDeviceDiscovery();
                if (device.state) {
                    info.dialogText = "Searching...";
                    info.visible = true;
                }
            }
        }

        Image {
            id: bluetoothImage
            x: 20
            y: 10
            width: 50
            height: 50
            source: "resources/bluetooth.png"
            fillMode: Image.PreserveAspectFit
        }
    }

    Dialog {
        id: info
        anchors.centerIn: parent
        visible: false
    }

    ListView {
        id: theListView
        width: parent.width
        clip: true

        anchors.top: menu.bottom
        anchors.bottom: parent.bottom
        model: device.devicesList

        delegate: Rectangle {
            id: box
            height:100
            width: theListView.width
            color: "lightsteelblue"
            border.width: 2
            border.color: "black"
            radius: 5

            Component.onCompleted: {
                info.visible = false;
            }

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    client.connect(modelData.deviceAddress);
                    pageLoader.source = "Client.qml"
                }
            }

            Label {
                id: deviceName
                textContent: modelData.deviceName
                anchors.top: parent.top
                anchors.topMargin: 5
            }

            Label {
                id: deviceAddress
                textContent: modelData.deviceAddress
                font.pointSize: deviceName.font.pointSize*0.7
                anchors.bottom: box.bottom
                anchors.bottomMargin: 5
            }
        }
    }

    Loader {
        id: pageLoader
        anchors.fill: parent
    }
}
